use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::Changed,
        system::{Query, Res},
    },
    transform::components::Transform,
};

use crate::{
    component::GridCell,
    error::GridError,
    event::{GridEvent, GridOp},
    resource::Grid,
};

pub(crate) fn check_grid(
    grid: Res<Grid>,
    grid_elements: Query<(Entity, &Transform, Option<&GridCell>), Changed<Transform>>,
    mut events: EventWriter<GridEvent>,
) {
    for (entity, transform, cell) in grid_elements.iter() {
        match grid.world_to_grid(transform.translation) {
            Ok(new_cell) => {
                let Some(&GridCell(cell)) = cell else {
                    events.send(GridEvent {
                        entity,
                        op: GridOp::Insert { to: new_cell },
                    });
                    continue;
                };
                if new_cell != cell {
                    events.send(GridEvent {
                        entity,
                        op: GridOp::Update {
                            from: cell,
                            to: new_cell,
                        },
                    });
                };
            }
            Err(GridError::OutOfBounds(_)) => {
                if let Some(&GridCell(cell)) = cell {
                    events.send(GridEvent {
                        entity,
                        op: GridOp::Remove { from: cell },
                    });
                }
            }
            _ => (),
        };
    }
}
