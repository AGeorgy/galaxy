use bevy::prelude::*;
use bevy_pancam::PanCam;

use super::components;
use super::density_wave_resource;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());

    let galaxy_settings = components::GalaxySettingsResource {
        radius: 16000,
        bulge_radius: 0.25,
        angular_offset: 0.0003,
        inner_excentricity: 0.8,
        outter_excentricity: 0.85,
        ellipse_disturbances: 0,
        ellipse_disturbances_damping: 40,
        count_stars: 60000,
        count_h2: 500,
        has_dark_matter: true,
        base_temp: 4000,
        dust_render_size: 70,
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

    commands.insert_resource(galaxy_settings);
    commands.insert_resource(density_wave);

    let sprite_handle = assets.load("particle.png");
    let spacing = 5.0;
    let w = 100;
    let h = 100;
    for i in 0..w {
        for j in 0..h {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1., 0.8, 0.6, 1.),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                texture: sprite_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    i as f32 * spacing - w as f32 * spacing / 2.,
                    j as f32 * spacing - h as f32 * spacing / 2.,
                    0.,
                )),
                ..default()
            });
        }
    }
}

pub fn create_stars(
    mut commands: Commands,
    density_wave: Res<density_wave_resource::DensityWaveResource>,
) {
}
