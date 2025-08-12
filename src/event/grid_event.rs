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

impl std::fmt::Display for GridOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GridOp::*;
        match self {
            Insert { to } => write!(f, "Insert {{ to: ({}, {}) }}", to.x, to.y),
            Remove { from } => write!(f, "Remove {{ from: ({}, {}) }}", from.x, from.y),
            Update { from, to } => write!(
                f,
                "Update {{ from: ({}, {}), to: ({}, {}) }}",
                from.x, from.y, to.x, to.y
            ),
        }
    }
}
