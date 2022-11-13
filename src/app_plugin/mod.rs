use bevy::{prelude::*, winit::WinitSettings};

use super::pan_cam;

pub mod density_wave;
mod dust_fade_system;
pub mod galaxy_setting_component;
mod lod_setting_resource;
mod setup_system;
mod star_component;
mod stars_lod_system;
mod update_color_system;
mod update_stars_system;
mod update_transform_system;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::desktop_app())
            .add_startup_system(setup_system::setup)
            .add_system(update_stars_system::update_stars)
            // Update transform and color if changed
            .add_system(update_transform_system::update_transform)
            .add_system(update_color_system::update_color)
            // Update bloom while zooming
            .add_system_to_stage(
                CoreStage::PostUpdate,
                stars_lod_system::update_stars_visibility,
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                stars_lod_system::update_other_visibility,
            )
            .add_system(dust_fade_system::update_dust_fade);
    }
}
