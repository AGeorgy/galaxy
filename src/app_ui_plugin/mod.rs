use bevy::prelude::{App, Plugin};

use super::app_plugin;
mod components;
mod settings_ui_system;
mod setup_system;
mod statistics_system;

pub struct AppUIPlugin;

impl Plugin for AppUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system::setup_ui)
            .add_system(statistics_system::update_fps)
            .add_system(statistics_system::update_stars_count)
            .add_system(settings_ui_system::update_stars_count);
    }
}
