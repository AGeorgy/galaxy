use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;

use super::density_wave;
use super::galaxy_setting_component;
use super::pan_cam::PanCam;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR must be enabled on the camera
                ..default()
            },
            projection: OrthographicProjection {
                scale: 70.,
                ..default()
            },
            ..default()
        },
        // Maximum zoom settings
        BloomSettings {
            threshold: 0.19,
            knee: 0.35,
            scale: 5.0,
            intensity: 10.0,
        },
        PanCam {
            min_scale: 0.01,
            max_scale: Some(80.),
            ..default()
        },
    ));

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
        count_dusts: 40000,
        count_dusts_filaments: 40000,
        count_h2: 400,
        has_dark_matter: true,
        base_temp: 4000.,
        dust_render_size: 70.,
        seed: 1234567890,
        pert_n: 2,
        pert_amp: 40,
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

    commands.spawn((Name::from("Settings"), galaxy_settings, density_wave));

    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 18.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
    );
}
