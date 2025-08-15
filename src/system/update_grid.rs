use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{Changed, With},
        system::{Commands, Query, QueryLens, ResMut},
    },
    transform::components::Transform,
};

use crate::{
    component::GridCell,
    error::GridError,
    event::{GridEvent, GridOperation, TransformGridEvent},
    resource::Grid,
};

pub(crate) fn update_grid<Marker: Component, const N: usize>(
    mut commands: Commands,
    mut grid: ResMut<Grid<Marker, N>>,
    mut transforms: Query<&Transform, With<Marker>>,
    mut changed_transforms: Query<&Transform, (Changed<Transform>, With<Marker>)>,
    mut grid_elements: Query<(Entity, Option<&mut GridCell>), With<Marker>>,
    mut grid_events: EventWriter<GridEvent>,
    mut transform_grid_events: EventReader<TransformGridEvent<Marker, N>>,
) {
    let mut grid_elements: QueryLens<(Entity, &Transform, Option<&mut GridCell>), With<Marker>> =
        if transform_grid_events.is_empty() {
            changed_transforms.join_filtered(&mut grid_elements)
        } else {
            for event in transform_grid_events.read() {
                if let Some(dimensions) = event.dimensions {
                    grid.set_dimensions(dimensions);
                };
                if let Some(spacing) = event.spacing {
                    grid.set_spacing(spacing);
                };
                if let Some(anchor) = event.anchor {
                    grid.set_anchor(anchor);
                };
            }
            grid.reset();
            transforms.join_filtered(&mut grid_elements)
        };
    for (entity, transform, current_cell) in grid_elements.query().iter_mut() {
        match grid.world_to_grid(transform.translation) {
            Ok(new_cell) => {
                let Some(mut current_cell) = current_cell else {
                    if grid.insert(entity, new_cell).is_ok() {
                        commands.entity(entity).insert(GridCell(new_cell));
                        grid_events.send(GridEvent {
                            entity,
                            operation: GridOperation::Insert { to: new_cell },
                        });
                    }
                    continue;
                };
                if new_cell != current_cell.0 {
                    let _ = grid.update(entity, current_cell.0, new_cell);
                    grid_events.send(GridEvent {
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
                if let Some(current_cell) = current_cell {
                    let _ = grid.remove(entity, current_cell.0);
                    commands.entity(entity).remove::<GridCell>();
                    grid_events.send(GridEvent {
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
