use bevy::prelude::*;
use rand::prelude::*;

use super::star_component::Star;
use super::star_component::{self, StarType};
use super::{density_wave, galaxy_setting_component};

pub fn update_stars(
    mut commands: Commands,
    settings_query: Query<
        (
            &galaxy_setting_component::GalaxySettings,
            &density_wave::DensityWave,
        ),
        Or<(
            Changed<galaxy_setting_component::GalaxySettings>,
            Changed<density_wave::DensityWave>,
        )>,
    >,
    mut stars: Query<Entity, With<star_component::Star>>,
    assets: Res<AssetServer>,
) {
    for (galaxy_setting, density_wave) in &settings_query {
        if !stars.is_empty() {
            // delete all
            for star_entity in &mut stars {
                commands.entity(star_entity).despawn();
            }
        }

        let stars = create_all_objects(galaxy_setting, density_wave, assets.load("particle.png"));
        commands.spawn_batch(stars);
    }
}

fn create_all_objects(
    galaxy_setting: &galaxy_setting_component::GalaxySettings,
    density_wave: &density_wave::DensityWave,
    sprite_handle: Handle<Image>,
) -> Vec<star_component::StarSpriteBundle> {
    let rnd = StdRng::seed_from_u64(galaxy_setting.seed);
    let mut stars: Vec<star_component::StarSpriteBundle> = vec![];

    create_stars(galaxy_setting, density_wave, sprite_handle, rnd, &mut stars);
    stars
}

fn create_stars(
    galaxy_setting: &galaxy_setting_component::GalaxySettings,
    density_wave: &density_wave::DensityWave,
    sprite_handle: Handle<Image>,
    mut rnd: StdRng,
    stars: &mut Vec<star_component::StarSpriteBundle>,
) {
    // First star ist the black hole at the centre
    let star = star_component::StarSpriteBundle {
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
            custom_size: Some(Vec2::new(0.5, 0.5)),
            ..default()
        },
        ..default()
    };
    stars.push(star);

    // Initialize stars
    for i in 1..galaxy_setting.count_stars {
        let rad: f32 = density_wave.val_from_prob(rnd.gen());
        let mut star_sprite = star_component::StarSpriteBundle {
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
                custom_size: Some(Vec2::new(1.5, 1.5)),
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
