use bevy::{
    ecs::entity::Entity,
    math::{IVec2, UVec2},
};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GridError {
    #[error("cell {0} is outside the grid")]
    OutOfBounds(IVec2),
    #[error("cell {0} not found")]
    CellNotFound(UVec2),
    #[error("entity {0:?} not found")]
    EntityNotFound(Entity),
}
