use bevy::ecs::{entity::Entity, event::Event};
use glam::UVec2;

#[derive(Clone, Copy, Debug, Event)]
pub struct GridEvent {
    pub entity: Entity,
    pub op: GridOp,
}

#[derive(Clone, Copy, Debug)]
pub enum GridOp {
    Insert { to: UVec2 },
    Remove { from: UVec2 },
    Update { from: UVec2, to: UVec2 },
}
