use bevy::ecs::component::Component;
use glam::UVec2;

#[derive(Component, Debug, Default)]
pub struct GridCell(pub UVec2);
