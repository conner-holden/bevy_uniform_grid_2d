use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    event::GridEvent,
    system::{check_grid, setup_grid, update_grid},
};

#[derive(Default)]
pub struct UniformGrid2dPlugin;

impl Plugin for UniformGrid2dPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<GridEvent>()
            .add_systems(Update, (setup_grid, check_grid, update_grid).chain());
    }
}
