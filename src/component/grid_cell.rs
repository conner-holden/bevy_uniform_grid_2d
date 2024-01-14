use bevy::{ecs::component::Component, math::UVec2};

#[derive(Component, Debug, Default)]
pub struct GridCell(pub UVec2);
