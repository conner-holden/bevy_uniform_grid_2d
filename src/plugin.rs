use bevy::app::{Plugin, Update};

use crate::{
    event::GridEvent,
    system::{update_debug_grid_lines, update_grid},
};

#[derive(Default)]
pub struct UniformGrid2dPlugin {
    pub debug: bool,
}

impl Plugin for UniformGrid2dPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<GridEvent>()
            .add_systems(Update, update_grid);
        if self.debug {
            app.add_systems(Update, update_debug_grid_lines);
        }
    }
}
