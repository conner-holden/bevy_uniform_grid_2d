use bevy::ecs::{
        event::EventReader,
        system::{Commands, ResMut},
    };

use crate::{
    event::{GridEvent, GridOp},
    resource::Grid,
};

pub(crate) fn update_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut events: EventReader<GridEvent>,
) {
    use GridOp::*;
    for &GridEvent { entity, op } in events.read() {
        let _ = match op {
            Insert { to } => grid.insert(&mut commands, entity, to),
            Remove { from } => grid.remove(&mut commands, entity, from),
            Update { from, to } => grid.update(&mut commands, entity, from, to),
        };
    }
}
