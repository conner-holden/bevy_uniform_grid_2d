use bevy::ecs::{
    entity::Entity,
    system::{Commands, Resource},
};
use glam::{IVec2, UVec2, Vec2, Vec3, Vec3Swizzles};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::{error::GridError, prelude::GridCell};

#[derive(Default, Resource)]
pub struct Grid {
    /// Shape of the grid in cell units
    pub dimensions: UVec2,
    /// Shape of each grid cell
    pub spacing: UVec2,
    /// Point in world space to anchor the grid. Defaults to the origin.
    pub anchor: Vec2,
    data: FxHashMap<UVec2, SmallVec<[Entity; 4]>>,
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

    pub fn splat(size: u32) -> Self {
        Self {
            dimensions: UVec2::splat(size),
            ..Default::default()
        }
    }

    pub fn with_dimensions(mut self, dimensions: UVec2) -> Self {
        self.dimensions = dimensions;
        self
    }

    pub fn with_spacing(mut self, spacing: UVec2) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn insert(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        cell: UVec2,
    ) -> Result<(), GridError> {
        if !self.is_in_bounds(cell) {
            return Err(GridError::OutOfBounds(cell));
        }
        self.data.entry(cell).or_default().push(entity);
        commands.entity(entity).insert(GridCell(cell));
        Ok(())
    }

    pub fn insert_at_world_position(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        translation: Vec3,
    ) -> Result<UVec2, GridError> {
        let cell = self.world_to_grid(translation)?;
        self.data.entry(cell).or_default().push(entity);
        commands.entity(entity).insert(GridCell(cell));
        Ok(cell)
    }

    pub fn get(&self, cell: UVec2) -> impl Iterator<Item = Entity> {
        self.data
            .get(&cell)
            .map_or([].iter().copied(), |v| v.iter().copied())
    }

    pub fn get_neighbors(&self, cell: UVec2) -> impl Iterator<Item = Entity> {
        self.get_cell_neighbors(cell)
            .into_iter()
            .flat_map(move |neighbor_cell| self.get(neighbor_cell))
    }

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

    pub fn remove(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        cell: UVec2,
    ) -> Result<(), GridError> {
        self.remove_from_grid(entity, cell)?;
        commands.entity(entity).insert(GridCell(cell));
        Ok(())
    }

    pub fn update(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
        current_cell: UVec2,
        new_cell: UVec2,
    ) -> Result<(), GridError> {
        if !self.is_in_bounds(new_cell) {
            return Err(GridError::OutOfBounds(new_cell));
        }
        // Remove from current cell
        self.remove_from_grid(entity, current_cell)?;
        // Add to new cell
        self.data.entry(new_cell).or_default().push(entity);
        commands.entity(entity).insert(GridCell(new_cell));
        Ok(())
    }

    fn is_in_bounds(&self, cell: UVec2) -> bool {
        cell.cmplt(self.dimensions).all()
    }

    pub fn world_to_grid(&self, translation: Vec3) -> Result<UVec2, GridError> {
        let cell: UVec2 = (translation.xy() - self.anchor).floor().as_uvec2();
        if !self.is_in_bounds(cell) {
            return Err(GridError::OutOfBounds(cell));
        }
        Ok(cell)
    }

    fn get_cell_neighbors(&self, cell: UVec2) -> Vec<UVec2> {
        let cell_i = cell.as_ivec2();
        let mut neighbors = Vec::with_capacity(8);

        // Generate all 9 combinations using bit patterns (0-8), skip center (4)
        for pattern in 0u8..9 {
            if pattern == 4 {
                continue;
            } // Skip center (1,1) -> pattern 4

            // Convert pattern to x,y offsets: pattern = y*3 + x
            let x_offset = (pattern % 3) as i32 - 1; // 0,1,2 -> -1,0,1
            let y_offset = (pattern / 3) as i32 - 1; // 0,1,2 -> -1,0,1

            let neighbor = (cell_i + IVec2::new(x_offset, y_offset)).as_uvec2();
            if self.is_in_bounds(neighbor) {
                neighbors.push(neighbor);
            }
        }

        neighbors
    }
}
