use bevy::ecs::{entity::Entity, system::Resource};
use glam::{IVec2, UVec2, Vec2, Vec3, Vec3Swizzles};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::error::GridError;

/// Iterator over neighboring grid cells
pub struct NeighborIterator {
    cell: IVec2,
    dimensions: UVec2,
    pattern: u8,
}

impl Iterator for NeighborIterator {
    type Item = UVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pattern < 9 {
            if self.pattern == 4 {
                self.pattern += 1;
                continue; // Skip center cell
            }

            let x_offset = (self.pattern % 3) as i32 - 1;
            let y_offset = (self.pattern / 3) as i32 - 1;
            let neighbor = self.cell + IVec2::new(x_offset, y_offset);
            self.pattern += 1;

            // Check bounds
            if neighbor.cmpge(IVec2::ZERO).all() && neighbor.cmplt(self.dimensions.as_ivec2()).all() {
                return Some(neighbor.as_uvec2());
            }
        }
        None
    }
}

#[derive(Default, Resource)]
pub struct Grid {
    /// Shape of the grid in cell units.
    pub dimensions: UVec2,
    /// Shape of each grid cell in world-space units.
    pub spacing: UVec2,
    /// Point in world space to anchor the grid. Defaults to the origin.
    pub anchor: Vec2,
    pub data: FxHashMap<UVec2, SmallVec<[Entity; 4]>>,
}

impl Grid {
    pub fn new(dimensions: UVec2, spacing: UVec2, anchor: Vec2) -> Self {
        Self {
            dimensions,
            spacing,
            anchor,
            data: FxHashMap::default(),
        }
    }

    /// Consruct a square grid with length and width equal to `size`.
    pub fn splat(size: u32) -> Self {
        Self {
            dimensions: UVec2::splat(size),
            ..Default::default()
        }
    }

    /// Builder method to set the grid's `dimensions`.
    pub fn with_dimensions(mut self, dimensions: UVec2) -> Self {
        self.dimensions = dimensions;
        self
    }

    /// Builder method to set the grid's `spacing`.
    pub fn with_spacing(mut self, spacing: UVec2) -> Self {
        self.spacing = spacing;
        self
    }

    /// Insert an `entity` into the grid at `cell` coordinate. Updates
    /// the `GridCell` component of the entity to reflect the change.
    #[inline(always)]
    pub fn insert(&mut self, entity: Entity, cell: UVec2) -> Result<(), GridError> {
        if !self.contains_cell(cell) {
            return Err(GridError::OutOfBounds(cell.as_ivec2()));
        }
        self.data.entry(cell).or_default().push(entity);
        Ok(())
    }

    /// Insert an `entity` into the grid at `translation` world-space coordinate.
    /// Updates the `GridCell` component of the entity to reflect the change.
    #[inline(always)]
    pub fn insert_at_world_position(
        &mut self,
        entity: Entity,
        translation: Vec3,
    ) -> Result<UVec2, GridError> {
        let cell = self.world_to_grid(translation)?;
        self.data.entry(cell).or_default().push(entity);
        Ok(cell)
    }

    #[inline(always)]
    pub fn get(&self, cell: UVec2) -> impl Iterator<Item = Entity> {
        self.data
            .get(&cell)
            .map_or([].iter().copied(), |v| v.iter().copied())
    }

    #[inline(always)]
    pub fn iter_neighbors(&self, cell: UVec2) -> impl Iterator<Item = Entity> + '_ {
        self.get_cell_neighbors(cell)
            .flat_map(move |neighbor_cell| self.get(neighbor_cell))
    }

    #[inline(always)]
    fn remove_from_grid(&mut self, entity: Entity, cell: UVec2) -> Result<(), GridError> {
        if let Some(entities) = self.data.get_mut(&cell) {
            if let Some(pos) = entities.iter().position(|&e| e == entity) {
                entities.swap_remove(pos);
                if entities.is_empty() {
                    self.data.remove(&cell);
                }
            } else {
                return Err(GridError::EntityNotFound(entity));
            }
        } else {
            return Err(GridError::CellNotFound(cell));
        }
        Ok(())
    }

    /// Remove an `entity` located at `cell` coordinate from the grid. Updates
    /// the `GridCell` component of the entity to reflect the change.
    #[inline(always)]
    pub fn remove(&mut self, entity: Entity, cell: UVec2) -> Result<(), GridError> {
        self.remove_from_grid(entity, cell)?;
        Ok(())
    }

    /// Change the grid cell coordinate of an `entity` from `current_cell` to
    /// `new_cell`. Updates the `GridCell` component of the entity to reflect
    /// the change.
    #[inline(always)]
    pub fn update(
        &mut self,
        entity: Entity,
        current_cell: UVec2,
        new_cell: UVec2,
    ) -> Result<(), GridError> {
        if !self.contains_cell(new_cell) {
            return Err(GridError::OutOfBounds(new_cell.as_ivec2()));
        }
        // Remove from current cell
        self.remove_from_grid(entity, current_cell)?;
        // Add to new cell
        self.data.entry(new_cell).or_default().push(entity);
        Ok(())
    }

    /// Return whether a `cell` coordinate is inside the grid. Cell coordinates
    /// have a minimum at (0,0) and an exclusive maximum at the grid's `dimensions`.
    /// Cell coordinates will always be non-negative because they are relative to
    /// grid space and not world space.
    #[inline(always)]
    pub fn contains_cell(&self, cell: UVec2) -> bool {
        cell.cmplt(self.dimensions).all()
    }

    /// Convert a `translation` in world space to a grid cell coordinate.
    #[inline(always)]
    pub fn world_to_grid(&self, translation: Vec3) -> Result<UVec2, GridError> {
        let cell: IVec2 = ((translation.xy() - self.anchor) / self.spacing.as_vec2())
            .floor()
            .as_ivec2();
        if cell.cmpge(self.dimensions.as_ivec2()).any() || cell.cmplt(IVec2::ZERO).any() {
            return Err(GridError::OutOfBounds(cell));
        }
        Ok(cell.as_uvec2())
    }

    /// Return an iterator over all valid neighboring cell coordinates.
    #[inline(always)]
    pub fn get_cell_neighbors(&self, cell: UVec2) -> NeighborIterator {
        NeighborIterator {
            cell: cell.as_ivec2(),
            dimensions: self.dimensions,
            pattern: 0,
        }
    }
}
