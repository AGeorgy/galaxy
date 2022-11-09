use bevy::prelude::*;
use bevy_pancam::PanCam;

use super::density_wave;
use super::galaxy_setting_component;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());

    let galaxy_settings = galaxy_setting_component::GalaxySettings {
        radius: 13000.,
        far_field_radius: 16000. * 2.,
        bulge_radius: 4000.,
        angular_offset: 0.0004,
        inner_excentricity: 0.85,
        outter_excentricity: 0.95,
        ellipse_disturbances: 0,
        ellipse_disturbances_damping: 40,
        count_stars: 40000,
        //count_h2: 500,
        has_dark_matter: true,
        base_temp: 4000.,
        dust_render_size: 70,
        seed: 1234567890,
    };
    let wave_steps = 1000;
    let mut density_wave = density_wave::DensityWave {
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

    commands
        .spawn()
        .insert(Name::from("Settings"))
        .insert(galaxy_settings)
        .insert(density_wave);
}
