use bevy::ecs::entity::Entity;
use glam::UVec2;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GridError {
    #[error("cell {0} is outside the grid")]
    OutOfBounds(UVec2),
    #[error("cell {0} not found")]
    CellNotFound(UVec2),
    #[error("entity {0} not found")]
    EntityNotFound(Entity),
}
