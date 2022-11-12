use bevy::prelude::*;

use super::pan_cam;

mod density_wave;
mod galaxy_setting_component;
mod setup_system;
mod star_component;
mod update_stars_system;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::BLACK))
            .register_type::<galaxy_setting_component::GalaxySettings>()
            .register_type::<density_wave::DensityWave>()
            .register_type::<star_component::Star>()
            .register_type::<star_component::StarType>()
            .add_startup_system(setup_system::setup)
            .add_system(update_stars_system::update_stars)
            .add_system(update_stars_system::update_transform)
            .add_system(update_stars_system::update_bloom_settings)
            // Camera
            // .add_system(pan_cam::camera_movement)
            // .add_system(pan_cam::camera_zoom)
            // Loging
            .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
    }
}
