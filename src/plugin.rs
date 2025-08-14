use std::marker::PhantomData;

use bevy::{
    app::{Plugin, Update},
    ecs::component::Component,
};

use crate::{
    event::GridEvent,
    system::{update_debug_grid_lines, update_grid},
};

pub struct UniformGrid2dPlugin<Marker: Component, const N: usize = 4> {
    pub debug: bool,
    marker: PhantomData<Marker>,
}

impl<Marker: Component, const N: usize> UniformGrid2dPlugin<Marker, N> {
    pub fn debug(mut self, value: bool) -> Self {
        self.debug = value;
        self
    }
}

impl<Marker: Component, const N: usize> Default for UniformGrid2dPlugin<Marker, N> {
    fn default() -> Self {
        Self {
            debug: false,
            marker: PhantomData,
        }
    }
}

impl<Marker: Component, const N: usize> Plugin for UniformGrid2dPlugin<Marker, N> {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<GridEvent>()
            .add_systems(Update, update_grid::<Marker, N>);
        if self.debug {
            app.add_systems(Update, update_debug_grid_lines::<Marker, N>);
        }
    }
}
