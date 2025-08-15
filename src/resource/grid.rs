use std::marker::PhantomData;

use bevy::{
    ecs::{component::Component, entity::Entity, resource::Resource},
    math::{IVec2, UVec2, Vec2, Vec3, Vec3Swizzles},
};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::error::GridError;

#[derive(Resource)]
pub struct Grid<Marker: Component, const N: usize = 4> {
    /// Shape of the grid in cell units.
    pub(crate) dimensions: UVec2,
    /// Shape of each grid cell in world-space units.
    pub(crate) spacing: UVec2,
    /// Point in world space to anchor the grid. Defaults to the origin.
    pub(crate) anchor: Vec2,
    data: FxHashMap<UVec2, SmallVec<[Entity; N]>>,
    marker: PhantomData<Marker>,
}

impl<Marker: Component, const N: usize> Default for Grid<Marker, N> {
    fn default() -> Self {
        Self {
            dimensions: UVec2::ONE,
            spacing: UVec2::ONE,
            anchor: Vec2::ZERO,
            data: FxHashMap::default(),
            marker: PhantomData,
        }
    }
}

impl<Marker: Component, const N: usize> Grid<Marker, N> {
    /// Getter method for the grid's `dimensions`.
    pub fn dimensions(&self) -> UVec2 {
        self.dimensions
    }

    /// Getter method for the grid's `spacing`.
    pub fn spacing(&self) -> UVec2 {
        self.spacing
    }

    /// Getter method for the grid's `anchor`.
    pub fn anchor(&self) -> Vec2 {
        self.anchor
    }

    /// Builder method to set the grid's `dimensions`.
    pub fn with_dimensions(mut self, value: impl Into<UVec2>) -> Self {
        self.dimensions = value.into();
        self
    }

    /// Builder method to set the grid's `spacing`.
    pub fn with_spacing(mut self, value: impl Into<UVec2>) -> Self {
        self.spacing = value.into();
        self
    }

    /// Builder method to set the grid's `anchor`.
    pub fn with_anchor(mut self, value: impl Into<Vec2>) -> Self {
        self.anchor = value.into();
        self
    }

    /// Setter method for the grid's `dimensions`.
    pub fn set_dimensions(&mut self, value: impl Into<UVec2>) -> &mut Self {
        self.dimensions = value.into();
        self
    }

    /// Setter method for the grid's `spacing`.
    pub fn set_spacing(&mut self, value: impl Into<UVec2>) -> &mut Self {
        self.spacing = value.into();
        self
    }

    /// Setter method for the grid's `anchor`.
    pub fn set_anchor(&mut self, value: impl Into<Vec2>) -> &mut Self {
        self.anchor = value.into();
        self
    }

    pub fn reset(&mut self) {
        self.data = FxHashMap::default();
    }

    /// Insert an `entity` into the grid at `cell` coordinate. Updates
    /// the `GridCell` component of the entity to reflect the change.
    #[inline]
    pub fn insert(&mut self, entity: Entity, cell: UVec2) -> Result<(), GridError> {
        if !self.contains_cell(cell) {
            return Err(GridError::OutOfBounds(cell.as_ivec2()));
        }
        self.data.entry(cell).or_default().push(entity);
        Ok(())
    }

    /// Insert an `entity` into the grid at `translation` world-space coordinate.
    /// Updates the `GridCell` component of the entity to reflect the change.
    #[inline]
    pub fn insert_at_world_position(
        &mut self,
        entity: Entity,
        translation: Vec3,
    ) -> Result<UVec2, GridError> {
        let cell = self.world_to_grid(translation)?;
        self.data.entry(cell).or_default().push(entity);
        Ok(cell)
    }

    #[inline]
    pub fn get(&self, cell: UVec2) -> impl Iterator<Item = Entity> {
        self.data
            .get(&cell)
            .map_or([].iter().copied(), |v| v.iter().copied())
    }

    #[inline]
    pub fn iter_neighbors(&self, cell: UVec2) -> impl Iterator<Item = Entity> + '_ {
        self.get_cell_neighbors(cell)
            .flat_map(move |neighbor_cell| self.get(neighbor_cell))
    }

    #[inline]
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
    #[inline]
    pub fn remove(&mut self, entity: Entity, cell: UVec2) -> Result<(), GridError> {
        self.remove_from_grid(entity, cell)?;
        Ok(())
    }

    /// Change the grid cell coordinate of an `entity` from `current_cell` to
    /// `new_cell`. Updates the `GridCell` component of the entity to reflect
    /// the change.
    #[inline]
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
    #[inline]
    pub fn contains_cell(&self, cell: UVec2) -> bool {
        cell.cmplt(self.dimensions).all()
    }

    /// Convert a `translation` in world space to a grid cell coordinate.
    #[inline]
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
    #[inline]
    pub fn get_cell_neighbors(&self, cell: UVec2) -> GridCellIterator {
        GridCellIterator {
            cell: cell.as_ivec2(),
            dimensions: self.dimensions,
            pattern: 0,
        }
    }
}

pub struct GridCellIterator {
    cell: IVec2,
    dimensions: UVec2,
    pattern: u8,
}

impl Iterator for GridCellIterator {
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
            if neighbor.cmpge(IVec2::ZERO).all() && neighbor.cmplt(self.dimensions.as_ivec2()).all()
            {
                return Some(neighbor.as_uvec2());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::component::Component;

    #[derive(Component)]
    struct TestMarker;

    #[test]
    fn test_grid_default() {
        let grid = Grid::<TestMarker>::default();
        assert_eq!(grid.dimensions(), UVec2::ONE);
        assert_eq!(grid.spacing(), UVec2::ONE);
        assert_eq!(grid.anchor(), Vec2::ZERO);
    }

    #[test]
    fn test_grid_builders() {
        let grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(20, 15))
            .with_spacing(UVec2::new(16, 16))
            .with_anchor(Vec2::new(10.0, 5.0));

        assert_eq!(grid.dimensions(), UVec2::new(20, 15));
        assert_eq!(grid.spacing(), UVec2::new(16, 16));
        assert_eq!(grid.anchor(), Vec2::new(10.0, 5.0));
    }

    #[test]
    fn test_contains_cell() {
        let grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));

        assert!(grid.contains_cell(UVec2::new(0, 0)));
        assert!(grid.contains_cell(UVec2::new(9, 9)));
        assert!(grid.contains_cell(UVec2::new(5, 5)));

        assert!(!grid.contains_cell(UVec2::new(10, 9)));
        assert!(!grid.contains_cell(UVec2::new(9, 10)));
        assert!(!grid.contains_cell(UVec2::new(10, 10)));
    }

    #[test]
    fn test_world_to_grid() {
        let grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));

        assert_eq!(
            grid.world_to_grid(Vec3::new(0.0, 0.0, 0.0)).unwrap(),
            UVec2::new(0, 0)
        );
        assert_eq!(
            grid.world_to_grid(Vec3::new(32.0, 32.0, 0.0)).unwrap(),
            UVec2::new(1, 1)
        );
        assert_eq!(
            grid.world_to_grid(Vec3::new(31.0, 31.0, 0.0)).unwrap(),
            UVec2::new(0, 0)
        );
        assert_eq!(
            grid.world_to_grid(Vec3::new(288.0, 288.0, 0.0)).unwrap(),
            UVec2::new(9, 9)
        );

        // Test out of bounds
        assert!(grid.world_to_grid(Vec3::new(-1.0, 0.0, 0.0)).is_err());
        assert!(grid.world_to_grid(Vec3::new(321.0, 0.0, 0.0)).is_err());
    }

    #[test]
    fn test_world_to_grid_with_anchor() {
        let grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32))
            .with_anchor(Vec2::new(16.0, 16.0));

        // With anchor at (16, 16), world position (16, 16) should be grid cell (0, 0)
        assert_eq!(
            grid.world_to_grid(Vec3::new(16.0, 16.0, 0.0)).unwrap(),
            UVec2::new(0, 0)
        );
        assert_eq!(
            grid.world_to_grid(Vec3::new(48.0, 48.0, 0.0)).unwrap(),
            UVec2::new(1, 1)
        );

        // Test out of bounds
        assert!(grid.world_to_grid(Vec3::new(0.0, 16.0, 0.0)).is_err());
    }

    #[test]
    fn test_insert_and_get() {
        let mut grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));
        let entity = Entity::from_raw(42);

        // Insert entity at valid cell
        assert!(grid.insert(entity, UVec2::new(5, 5)).is_ok());

        // Check entity is in the cell
        let entities: Vec<Entity> = grid.get(UVec2::new(5, 5)).collect();
        assert_eq!(entities, vec![entity]);

        // Check empty cell returns empty iterator
        let entities: Vec<Entity> = grid.get(UVec2::new(0, 0)).collect();
        assert!(entities.is_empty());

        // Test out of bounds insert
        assert!(grid.insert(entity, UVec2::new(10, 10)).is_err());
    }

    #[test]
    fn test_insert_at_world_position() {
        let mut grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));
        let entity = Entity::from_raw(42);

        // Insert at world position
        let cell = grid
            .insert_at_world_position(entity, Vec3::new(160.0, 160.0, 0.0))
            .unwrap();
        assert_eq!(cell, UVec2::new(5, 5));

        // Verify entity is in the correct cell
        let entities: Vec<Entity> = grid.get(UVec2::new(5, 5)).collect();
        assert_eq!(entities, vec![entity]);

        // Test out of bounds
        assert!(
            grid.insert_at_world_position(entity, Vec3::new(-1.0, 0.0, 0.0))
                .is_err()
        );
    }

    #[test]
    fn test_remove() {
        let mut grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));
        let entity = Entity::from_raw(42);

        // Insert and then remove
        grid.insert(entity, UVec2::new(5, 5)).unwrap();
        assert!(grid.remove(entity, UVec2::new(5, 5)).is_ok());

        // Check entity is removed
        let entities: Vec<Entity> = grid.get(UVec2::new(5, 5)).collect();
        assert!(entities.is_empty());

        // Test removing non-existent entity
        assert!(grid.remove(Entity::from_raw(99), UVec2::new(5, 5)).is_err());

        // Test removing from empty cell
        assert!(grid.remove(entity, UVec2::new(0, 0)).is_err());
    }

    #[test]
    fn test_update() {
        let mut grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));
        let entity = Entity::from_raw(42);

        // Insert entity
        grid.insert(entity, UVec2::new(5, 5)).unwrap();

        // Move entity to new cell
        assert!(
            grid.update(entity, UVec2::new(5, 5), UVec2::new(7, 7))
                .is_ok()
        );

        // Check entity is in new cell
        let entities: Vec<Entity> = grid.get(UVec2::new(7, 7)).collect();
        assert_eq!(entities, vec![entity]);

        // Check entity is removed from old cell
        let entities: Vec<Entity> = grid.get(UVec2::new(5, 5)).collect();
        assert!(entities.is_empty());

        // Test moving to out of bounds
        assert!(
            grid.update(entity, UVec2::new(7, 7), UVec2::new(10, 10))
                .is_err()
        );

        // Test moving non-existent entity
        assert!(
            grid.update(Entity::from_raw(99), UVec2::new(7, 7), UVec2::new(8, 8))
                .is_err()
        );
    }

    #[test]
    fn test_multiple_entities_same_cell() {
        let mut grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));
        let entity1 = Entity::from_raw(42);
        let entity2 = Entity::from_raw(43);
        let entity3 = Entity::from_raw(44);

        // Insert multiple entities in same cell
        grid.insert(entity1, UVec2::new(5, 5)).unwrap();
        grid.insert(entity2, UVec2::new(5, 5)).unwrap();
        grid.insert(entity3, UVec2::new(5, 5)).unwrap();

        // Check all entities are in the cell
        let mut entities: Vec<Entity> = grid.get(UVec2::new(5, 5)).collect();
        entities.sort_by_key(|e| e.index());
        assert_eq!(entities, vec![entity1, entity2, entity3]);

        // Remove one entity
        grid.remove(entity2, UVec2::new(5, 5)).unwrap();

        // Check remaining entities
        let mut entities: Vec<Entity> = grid.get(UVec2::new(5, 5)).collect();
        entities.sort_by_key(|e| e.index());
        assert_eq!(entities, vec![entity1, entity3]);
    }

    #[test]
    fn test_grid_cell_iterator() {
        let grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(5, 5))
            .with_spacing(UVec2::new(32, 32));

        // Test center cell (2,2) - should have 8 neighbors
        let neighbors: Vec<UVec2> = grid.get_cell_neighbors(UVec2::new(2, 2)).collect();
        assert_eq!(neighbors.len(), 8);

        let expected = vec![
            UVec2::new(1, 1),
            UVec2::new(2, 1),
            UVec2::new(3, 1),
            UVec2::new(1, 2),
            UVec2::new(3, 2),
            UVec2::new(1, 3),
            UVec2::new(2, 3),
            UVec2::new(3, 3),
        ];

        for expected_neighbor in expected {
            assert!(neighbors.contains(&expected_neighbor));
        }

        // Test corner cell (0,0) - should have 3 neighbors
        let neighbors: Vec<UVec2> = grid.get_cell_neighbors(UVec2::new(0, 0)).collect();
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&UVec2::new(1, 0)));
        assert!(neighbors.contains(&UVec2::new(0, 1)));
        assert!(neighbors.contains(&UVec2::new(1, 1)));

        // Test edge cell (4,2) - should have 5 neighbors
        let neighbors: Vec<UVec2> = grid.get_cell_neighbors(UVec2::new(4, 2)).collect();
        assert_eq!(neighbors.len(), 5);
    }

    #[test]
    fn test_iter_neighbors() {
        let mut grid = Grid::<TestMarker>::default()
            .with_dimensions(UVec2::new(10, 10))
            .with_spacing(UVec2::new(32, 32));
        let entity1 = Entity::from_raw(42);
        let entity2 = Entity::from_raw(43);
        let entity3 = Entity::from_raw(44);

        // Insert entities in neighboring cells
        grid.insert(entity1, UVec2::new(4, 4)).unwrap();
        grid.insert(entity2, UVec2::new(5, 4)).unwrap();
        grid.insert(entity3, UVec2::new(7, 7)).unwrap();

        // Get neighbors of center cell (5,5)
        let neighbors: Vec<Entity> = grid.iter_neighbors(UVec2::new(5, 5)).collect();

        // Should find entity1 and entity2
        assert!(neighbors.contains(&entity1));
        assert!(neighbors.contains(&entity2));
        assert!(!neighbors.contains(&entity3)); // Not adjacent to (5,5) - at (7,7)
    }
}
