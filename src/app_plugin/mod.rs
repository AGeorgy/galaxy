use bevy::prelude::*;

mod components;
mod density_wave_resource;
mod systems;

pub(crate) struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let galaxy_settings = components::GalaxySettingsResource {
            radius: 16000.,
            far_field_radius: 16000. * 2.,
            bulge_radius: 0.25,
            angular_offset: 0.0003,
            inner_excentricity: 0.8,
            outter_excentricity: 0.85,
            ellipse_disturbances: 0,
            ellipse_disturbances_damping: 40,
            count_stars: 60000,
            count_h2: 500,
            has_dark_matter: true,
            base_temp: 4000.,
            dust_render_size: 70,
            seed: 1234567890,
        };
        let wave_steps = 1000;
        let mut density_wave = density_wave_resource::DensityWaveResource {
            min: 0.,
            max: galaxy_settings.radius as f32 * 2.,
            steps: wave_steps,
            i0: 1.,
            k: 0.02,
            a: galaxy_settings.radius as f32 / 3.,
            bulge_radius: galaxy_settings.bulge_radius,
            m1: Vec::with_capacity(wave_steps.try_into().unwrap()),
            y1: Vec::with_capacity(wave_steps.try_into().unwrap()),
            x1: Vec::with_capacity(wave_steps.try_into().unwrap()),
            m2: Vec::with_capacity(wave_steps.try_into().unwrap()),
            y2: Vec::with_capacity(wave_steps.try_into().unwrap()),
            x2: Vec::with_capacity(wave_steps.try_into().unwrap()),
        };
        density_wave.build();

        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(galaxy_settings)
            .insert_resource(density_wave)
            .add_startup_system(systems::setup)
            .add_startup_system(systems::create_stars)
            // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
            // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_system(systems::update_position);
    }
}
