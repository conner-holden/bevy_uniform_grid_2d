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

pub(crate) fn update_grid<Marker: Component>(
    mut commands: Commands,
    mut grid: ResMut<Grid<Marker>>,
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
