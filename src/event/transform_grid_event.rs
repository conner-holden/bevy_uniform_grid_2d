use std::marker::PhantomData;

use bevy::{
    ecs::{component::Component, event::Event},
    math::{UVec2, Vec2},
};

#[derive(Clone, Copy, Debug, Event)]
pub struct TransformGridEvent<Marker: Component, const N: usize = 4> {
    /// Shape of the grid in cell units.
    pub(crate) dimensions: Option<UVec2>,
    /// Shape of each grid cell in world-space units.
    pub(crate) spacing: Option<Vec2>,
    /// Point in world space to anchor the grid. Defaults to the origin.
    pub(crate) anchor: Option<Vec2>,
    marker: PhantomData<Marker>,
}

impl<Marker: Component, const N: usize> TransformGridEvent<Marker, N> {
    /// Getter method for the grid's `dimensions`.
    pub fn dimensions(&self) -> Option<UVec2> {
        self.dimensions
    }

    /// Getter method for the grid's `spacing`.
    pub fn spacing(&self) -> Option<Vec2> {
        self.spacing
    }

    /// Getter method for the grid's `anchor`.
    pub fn anchor(&self) -> Option<Vec2> {
        self.anchor
    }

    /// Builder method to set the grid's `dimensions`.
    pub fn with_dimensions(mut self, value: impl Into<UVec2>) -> Self {
        self.dimensions = Some(value.into());
        self
    }

    /// Builder method to set the grid's `spacing`.
    pub fn with_spacing(mut self, value: impl Into<Vec2>) -> Self {
        self.spacing = Some(value.into());
        self
    }

    /// Builder method to set the grid's `anchor`.
    pub fn with_anchor(mut self, value: impl Into<Vec2>) -> Self {
        self.anchor = Some(value.into());
        self
    }
}

impl<Marker: Component, const N: usize> Default for TransformGridEvent<Marker, N> {
    fn default() -> Self {
        Self {
            dimensions: Some(UVec2::ONE),
            spacing: Some(Vec2::ONE),
            anchor: Some(Vec2::ZERO),
            marker: PhantomData,
        }
    }
}

impl<Marker: Component, const N: usize> std::fmt::Display for TransformGridEvent<Marker, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dimensions: String = self
            .dimensions
            .map(|d| format!("({}, {})", d.x, d.y))
            .unwrap_or("none".to_string());
        let spacing: String = self
            .spacing
            .map(|s| format!("({}, {})", s.x, s.y))
            .unwrap_or("none".to_string());
        let anchor: String = self
            .anchor
            .map(|a| format!("({}, {})", a.x, a.y))
            .unwrap_or("none".to_string());
        write!(
            f,
            "TransformGridEvent {{ dimensions={dimensions} spacing={spacing} anchor={anchor} }}"
        )
    }
}
