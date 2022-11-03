use bevy::prelude::*;
use bevy_pancam::PanCam;
use rand::prelude::*;

use super::components::Star;
use super::components::{self, StarType};
use super::density_wave_resource;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());
    // let sprite_handle = assets.load("particle.png");
    // let spacing = 5.0;
    // let w = 100;
    // let h = 100;
    // for i in 0..w {
    //     for j in 0..h {
    //         commands.spawn_bundle(SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::rgba(1., 0.8, 0.6, 1.),
    //                 custom_size: Some(Vec2::new(5.0, 5.0)),
    //                 ..default()
    //             },
    //             texture: sprite_handle.clone(),
    //             transform: Transform::from_translation(Vec3::new(
    //                 i as f32 * spacing - w as f32 * spacing / 2.,
    //                 j as f32 * spacing - h as f32 * spacing / 2.,
    //                 0.,
    //             )),
    //             ..default()
    //         });
    //     }
    // }
}

pub fn create_stars(
    mut commands: Commands,
    density_wave: Res<density_wave_resource::DensityWaveResource>,
    galaxy_setting: Res<components::GalaxySettingsResource>,
    assets: Res<AssetServer>,
) {
    let mut rnd = StdRng::seed_from_u64(galaxy_setting.seed);
    let mut stars = vec![];
    let sprite_handle = assets.load("particle.png");

    // First star ist the black hole at the centre
    let star = components::StarSpriteBundle {
        texture: sprite_handle.clone(),
        star: Star {
            theta0: 0.,
            vel_theta: 0.,
            tilt_angle: 0.,
            a: 0.,
            b: 0.,
            temp: 6000.,
            mag: 1.,
            star_type: StarType::Star,
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        ..default()
    };
    stars.push(star);

    // 1. Initialize the stars
    for i in 0..galaxy_setting.count_stars {
        let rad: f32 = density_wave.val_from_prob(rnd.gen());
        let mut star_sprite = components::StarSpriteBundle {
            texture: sprite_handle.clone(),
            star: Star {
                theta0: 360.0 * rnd.gen::<f32>(),
                vel_theta: galaxy_setting.get_orbital_velocity(rad),
                tilt_angle: galaxy_setting.get_angular_offset(rad),
                a: rad,
                b: rad * galaxy_setting.get_excentricity(rad),
                temp: (4000. * rnd.gen::<f32>() - 2000.) + 6000.,
                mag: 0.1 + 0.4 * rnd.gen::<f32>(),
                star_type: StarType::Star,
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        };

        // Make a small portion of the stars brighter
        if i < galaxy_setting.count_stars / 60 {
            star_sprite.star.mag = 1_f32.min(star_sprite.star.mag + 0.1 + rnd.gen::<f32>() * 0.4);
        }

        stars.push(star_sprite);
    }

    commands.spawn_batch(stars);
}

pub fn update_position(mut star_query: Query<(&Star, &mut Transform), Added<Transform>>) {
    for (star, mut transform) in &mut star_query {
        transform.translation = calculate_translation(star);
    }
}

const DEG_TO_RAD: f32 = 0.01745329251;

fn calculate_translation(star: &Star) -> Vec3 {
    let theta_actual = star.theta0 + star.vel_theta;
    let beta = -star.tilt_angle;
    let alpha = theta_actual * DEG_TO_RAD;
    let cosalpha = alpha.cos();
    let sinalpha = alpha.sin();
    let cosbeta = beta.cos();
    let sinbeta = beta.sin();

    Vec3 {
        x: (star.a * cosalpha * cosbeta - star.b * sinalpha * sinbeta),
        y: (star.a * cosalpha * sinbeta + star.b * sinalpha * cosbeta),
        z: 0.,
    }
}
