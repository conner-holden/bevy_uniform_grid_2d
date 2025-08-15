use std::marker::PhantomData;

use bevy::{
    app::{Plugin, Update},
    ecs::component::Component,
    math::{UVec2, Vec2},
};

use crate::{
    event::{GridEvent, TransformGridEvent},
    resource::Grid,
    system::{update_debug_grid_lines, update_grid},
};

pub struct UniformGrid2dPlugin<Marker: Component, const N: usize = 4> {
    dimensions: UVec2,
    spacing: UVec2,
    anchor: Vec2,
    debug: bool,
    marker: PhantomData<Marker>,
}

impl<Marker: Component, const N: usize> UniformGrid2dPlugin<Marker, N> {
    /// Builder method to enable debug mode.
    pub fn debug(mut self, value: bool) -> Self {
        self.debug = value;
        self
    }

    /// Builder method to set the shape of the grid in cell units.
    pub fn dimensions(mut self, value: impl Into<UVec2>) -> Self {
        self.dimensions = value.into();
        self
    }

    /// Builder method to set the shape of each grid cell in world-space units.
    pub fn spacing(mut self, value: impl Into<UVec2>) -> Self {
        self.spacing = value.into();
        self
    }

    /// Builder method to set the point in world space to anchor the grid. Defaults to the origin.
    pub fn anchor(mut self, value: impl Into<Vec2>) -> Self {
        self.anchor = value.into();
        self
    }
}

impl<Marker: Component, const N: usize> Default for UniformGrid2dPlugin<Marker, N> {
    fn default() -> Self {
        Self {
            dimensions: UVec2::ONE,
            spacing: UVec2::ONE,
            anchor: Vec2::ZERO,
            debug: false,
            marker: PhantomData,
        }
    }
}

impl<Marker: Component, const N: usize> Plugin for UniformGrid2dPlugin<Marker, N> {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<GridEvent>()
            .add_event::<TransformGridEvent<Marker, N>>()
            .insert_resource(
                Grid::<Marker, N>::default()
                    .with_dimensions(self.dimensions)
                    .with_spacing(self.spacing)
                    .with_anchor(self.anchor),
            )
            .add_systems(Update, update_grid::<Marker, N>);
        if self.debug {
            app.add_systems(Update, update_debug_grid_lines::<Marker, N>);
        }
    }
}
