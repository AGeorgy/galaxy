use std::fs;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;

use super::density_wave;
use super::galaxy_setting_component;
use super::lod_setting_resource;
use super::pan_cam::PanCam;

pub fn setup(mut commands: Commands) {
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

    commands.insert_resource(lod_setting_resource::LodSetting {
        stars_visibility: Vec2 { x: 0., y: 10. },
        other_visibility: Vec2 { x: 1., y: 90. },
        ..default()
    });

    // Reading settings from json
    const FILE_NAME_GALAXY: &str = "assets/galaxy_settings.json";
    let galaxy_settings: galaxy_setting_component::GalaxySettings =
        match fs::read_to_string(FILE_NAME_GALAXY) {
            Ok(file) => {
                info!("Setting {} is loaded", FILE_NAME_GALAXY);
                serde_json::from_str(&file).unwrap()
            }
            Err(_) => {
                warn!("Unable to read file. Setup default.");
                galaxy_setting_component::GalaxySettings {
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
                    count_h2_core: 400,
                    has_dark_matter: true,
                    base_temp: 4000.,
                    dust_render_size: 70.,
                    seed: 1234567890,
                    pert_n: 2,
                    pert_amp: 40,
                }
            }
        };

    const FILE_NAME_WAVE: &str = "assets/density_wave.json";
    let wave_steps = 1000;
    let mut density_wave: density_wave::DensityWave = match fs::read_to_string(FILE_NAME_WAVE) {
        Ok(file) => {
            info!("Setting {} is loaded", FILE_NAME_WAVE);
            serde_json::from_str(&file).unwrap()
        }
        Err(_) => {
            warn!("Unable to read file. Setup default.");
            density_wave::DensityWave {
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
            }
        }
    };
    density_wave.build();

    commands.insert_resource(galaxy_settings);
    commands.insert_resource(density_wave);

    // Writing settings to json
    // let json = serde_json::to_string_pretty(&galaxy_settings).unwrap();
    // fs::write("assets/galaxy_settings.json", json).expect("Unable to write file");
    // let json = serde_json::to_string_pretty(&density_wave).unwrap();
    // fs::write("assets/density_wave.json", json).expect("Unable to write file");
}
