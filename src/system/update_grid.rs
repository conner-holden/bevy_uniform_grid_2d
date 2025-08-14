use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::{Changed, With},
        system::{Commands, Query, ResMut},
    },
    transform::components::Transform,
};

use crate::{
    component::GridCell,
    error::GridError,
    event::{GridEvent, GridOperation},
    resource::Grid,
};

pub(crate) fn update_grid<Marker: Component, const N: usize>(
    mut commands: Commands,
    mut grid: ResMut<Grid<Marker, N>>,
    mut grid_elements: Query<
        (Entity, &Transform, Option<&mut GridCell>),
        (Changed<Transform>, With<Marker>),
    >,
    mut events: EventWriter<GridEvent>,
) {
    for (entity, transform, current_cell) in grid_elements.iter_mut() {
        match grid.world_to_grid(transform.translation) {
            Ok(new_cell) => {
                let Some(mut current_cell) = current_cell else {
                    if grid.insert(entity, new_cell).is_ok() {
                        commands.entity(entity).insert(GridCell(new_cell));
                        events.send(GridEvent {
                            entity,
                            operation: GridOperation::Insert { to: new_cell },
                        });
                    }
                    continue;
                };
                if new_cell != current_cell.0
                    && grid.update(entity, current_cell.0, new_cell).is_ok()
                {
                    events.send(GridEvent {
                        entity,
                        operation: GridOperation::Update {
                            from: current_cell.0,
                            to: new_cell,
                        },
                    });
                    current_cell.0 = new_cell;
                };
            }
            Err(GridError::OutOfBounds(_)) => {
                if let Some(current_cell) = current_cell
                    && grid.remove(entity, current_cell.0).is_ok()
                {
                    commands.entity(entity).remove::<GridCell>();
                    events.send(GridEvent {
                        entity,
                        operation: GridOperation::Remove {
                            from: current_cell.0,
                        },
                    });
                }
            }
            _ => (),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{
        app::{App, Update},
        ecs::component::Component,
        math::{UVec2, Vec2, Vec3},
    };

    #[derive(Component)]
    struct TestMarker;

    #[test]
    fn test_update_grid_insert_new_entity() {
        let mut app = App::new();

        // Add grid resource
        app.insert_resource(Grid::<TestMarker>::new(
            UVec2::new(10, 10),
            UVec2::new(32, 32),
            Vec2::ZERO,
        ));

        // Add events
        app.add_event::<GridEvent>();

        // Add system
        app.add_systems(Update, update_grid::<TestMarker>);

        // Create entity with Transform and TestMarker, but no GridCell
        let entity = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::new(64.0, 96.0, 0.0)),
                TestMarker,
            ))
            .id();

        // Verify entity doesn't have GridCell initially
        assert!(
            !app.world()
                .get_entity(entity)
                .unwrap()
                .contains::<GridCell>()
        );

        // Run system
        app.update();

        // Verify entity now has GridCell
        let grid_cell = app
            .world()
            .get_entity(entity)
            .unwrap()
            .get::<GridCell>()
            .unwrap();

        // Should be at grid cell (2, 3) since 64/32=2, 96/32=3
        assert_eq!(grid_cell.0, UVec2::new(2, 3));

        // Verify grid contains the entity
        let grid = app.world().resource::<Grid<TestMarker>>();
        let entities: Vec<Entity> = grid.get(UVec2::new(2, 3)).collect();
        assert_eq!(entities, vec![entity]);

        // Check that an Insert event was sent
        let mut event_reader = app
            .world_mut()
            .resource_mut::<bevy::ecs::event::Events<GridEvent>>();
        let events: Vec<GridEvent> = event_reader.drain().collect();
        assert_eq!(events.len(), 1);

        match events[0].operation {
            GridOperation::Insert { to } => assert_eq!(to, UVec2::new(2, 3)),
            _ => panic!("Expected Insert event"),
        }
        assert_eq!(events[0].entity, entity);
    }

    #[test]
    fn test_update_grid_move_existing_entity() {
        let mut app = App::new();

        // Add grid resource
        app.insert_resource(Grid::<TestMarker>::new(
            UVec2::new(10, 10),
            UVec2::new(32, 32),
            Vec2::ZERO,
        ));

        // Add events
        app.add_event::<GridEvent>();

        // Add system
        app.add_systems(Update, update_grid::<TestMarker>);

        // Create entity with existing GridCell
        let entity = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::new(64.0, 64.0, 0.0)),
                TestMarker,
                GridCell(UVec2::new(2, 2)),
            ))
            .id();

        // Manually insert entity into grid at its current position
        {
            let mut grid = app.world_mut().resource_mut::<Grid<TestMarker>>();
            grid.insert(entity, UVec2::new(2, 2)).unwrap();
        }

        // Move entity to new position
        {
            let mut entity_mut = app.world_mut().get_entity_mut(entity).unwrap();
            let mut transform = entity_mut.get_mut::<Transform>().unwrap();
            transform.translation = Vec3::new(128.0, 128.0, 0.0); // Should be grid cell (4, 4)
        }

        // Run system
        app.update();

        // Verify entity's GridCell was updated
        let grid_cell = app
            .world()
            .get_entity(entity)
            .unwrap()
            .get::<GridCell>()
            .unwrap();
        assert_eq!(grid_cell.0, UVec2::new(4, 4));

        // Verify grid was updated
        let grid = app.world().resource::<Grid<TestMarker>>();
        let old_entities: Vec<Entity> = grid.get(UVec2::new(2, 2)).collect();
        let new_entities: Vec<Entity> = grid.get(UVec2::new(4, 4)).collect();
        assert!(old_entities.is_empty());
        assert_eq!(new_entities, vec![entity]);

        // Check that an Update event was sent
        let mut event_reader = app
            .world_mut()
            .resource_mut::<bevy::ecs::event::Events<GridEvent>>();
        let events: Vec<GridEvent> = event_reader.drain().collect();
        assert_eq!(events.len(), 1);

        match events[0].operation {
            GridOperation::Update { from, to } => {
                assert_eq!(from, UVec2::new(2, 2));
                assert_eq!(to, UVec2::new(4, 4));
            }
            _ => panic!("Expected Update event"),
        }
        assert_eq!(events[0].entity, entity);
    }

    #[test]
    fn test_update_grid_remove_out_of_bounds_entity() {
        let mut app = App::new();

        // Add grid resource
        app.insert_resource(Grid::<TestMarker>::new(
            UVec2::new(10, 10),
            UVec2::new(32, 32),
            Vec2::ZERO,
        ));

        // Add events
        app.add_event::<GridEvent>();

        // Add system
        app.add_systems(Update, update_grid::<TestMarker>);

        // Create entity with existing GridCell
        let entity = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::new(64.0, 64.0, 0.0)),
                TestMarker,
                GridCell(UVec2::new(2, 2)),
            ))
            .id();

        // Manually insert entity into grid at its current position
        {
            let mut grid = app.world_mut().resource_mut::<Grid<TestMarker>>();
            grid.insert(entity, UVec2::new(2, 2)).unwrap();
        }

        // Move entity out of bounds
        {
            let mut entity_mut = app.world_mut().get_entity_mut(entity).unwrap();
            let mut transform = entity_mut.get_mut::<Transform>().unwrap();
            transform.translation = Vec3::new(-10.0, -10.0, 0.0); // Out of bounds
        }

        // Run system
        app.update();

        // Verify entity no longer has GridCell
        assert!(
            !app.world()
                .get_entity(entity)
                .unwrap()
                .contains::<GridCell>()
        );

        // Verify grid no longer contains entity
        let grid = app.world().resource::<Grid<TestMarker>>();
        let entities: Vec<Entity> = grid.get(UVec2::new(2, 2)).collect();
        assert!(entities.is_empty());

        // Check that a Remove event was sent
        let mut event_reader = app
            .world_mut()
            .resource_mut::<bevy::ecs::event::Events<GridEvent>>();
        let events: Vec<GridEvent> = event_reader.drain().collect();
        assert_eq!(events.len(), 1);

        match events[0].operation {
            GridOperation::Remove { from } => assert_eq!(from, UVec2::new(2, 2)),
            _ => panic!("Expected Remove event"),
        }
        assert_eq!(events[0].entity, entity);
    }

    #[test]
    fn test_update_grid_no_change() {
        let mut app = App::new();

        // Add grid resource
        app.insert_resource(Grid::<TestMarker>::new(
            UVec2::new(10, 10),
            UVec2::new(32, 32),
            Vec2::ZERO,
        ));

        // Add events
        app.add_event::<GridEvent>();

        // Add system
        app.add_systems(Update, update_grid::<TestMarker>);

        // Create entity with existing GridCell
        let entity = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::new(64.0, 64.0, 0.0)),
                TestMarker,
                GridCell(UVec2::new(2, 2)),
            ))
            .id();

        // Manually insert entity into grid at its current position
        {
            let mut grid = app.world_mut().resource_mut::<Grid<TestMarker>>();
            grid.insert(entity, UVec2::new(2, 2)).unwrap();
        }

        // Run system without changing transform
        app.update();

        // Verify entity still has same GridCell
        let grid_cell = app
            .world()
            .get_entity(entity)
            .unwrap()
            .get::<GridCell>()
            .unwrap();
        assert_eq!(grid_cell.0, UVec2::new(2, 2));

        // Verify no events were sent (entity didn't move)
        let mut event_reader = app
            .world_mut()
            .resource_mut::<bevy::ecs::event::Events<GridEvent>>();
        let events: Vec<GridEvent> = event_reader.drain().collect();
        assert_eq!(events.len(), 0);
    }
}
