use bevy::{
    ecs::{entity::Entity, event::Event},
    math::UVec2,
};

#[derive(Clone, Copy, Debug, Event)]
pub struct GridEvent {
    pub entity: Entity,
    pub operation: GridOperation,
}

impl std::fmt::Display for GridEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridEvent {{ entity={0} operation={1} }}",
            self.entity, self.operation
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GridOperation {
    Insert { to: UVec2 },
    Remove { from: UVec2 },
    Update { from: UVec2, to: UVec2 },
}

impl std::fmt::Display for GridOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GridOperation::*;
        match self {
            Insert { to } => write!(f, "Insert {{ none -> ({}, {}) }}", to.x, to.y),
            Remove { from } => write!(f, "Remove {{ ({}, {}) -> none }}", from.x, from.y),
            Update { from, to } => write!(
                f,
                "Update {{ ({}, {}) -> ({}, {}) }}",
                from.x, from.y, to.x, to.y
            ),
        }
    }
}
