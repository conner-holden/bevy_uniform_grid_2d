use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::Changed,
        system::{Commands, Query, ResMut},
    },
    transform::components::Transform,
};

use crate::{
    component::GridCell,
    error::GridError,
    event::{GridEvent, GridOp},
    resource::Grid,
};

pub(crate) fn update_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut grid_elements: Query<(Entity, &Transform, Option<&mut GridCell>), Changed<Transform>>,
    mut events: EventWriter<GridEvent>,
) {
    for (entity, transform, cell) in grid_elements.iter_mut() {
        match grid.world_to_grid(transform.translation) {
            Ok(new_cell) => {
                let Some(mut cell) = cell else {
                    if grid.insert(entity, new_cell).is_ok() {
                        commands.entity(entity).insert(GridCell(new_cell));
                        events.send(GridEvent {
                            entity,
                            op: GridOp::Insert { to: new_cell },
                        });
                    }
                    continue;
                };
                if new_cell != cell.0 && grid.update(entity, cell.0, new_cell).is_ok() {
                    events.send(GridEvent {
                        entity,
                        op: GridOp::Update {
                            from: cell.0,
                            to: new_cell,
                        },
                    });
                    cell.0 = new_cell;
                };
            }
            Err(GridError::OutOfBounds(_)) => {
                if let Some(cell) = cell {
                    if grid.remove(entity, cell.0).is_ok() {
                        commands.entity(entity).remove::<GridCell>();
                        events.send(GridEvent {
                            entity,
                            op: GridOp::Remove { from: cell.0 },
                        });
                    }
                }
            }
            _ => (),
        };
    }
}
