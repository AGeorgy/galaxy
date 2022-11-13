use bevy::prelude::*;

use super::{
    galaxy_setting_component,
    star_component::{self, Star},
};

pub fn update_transform(
    mut star_query: Query<
        (
            &Star,
            &mut Transform,
            &mut Sprite,
            AnyOf<(
                &star_component::StarTag,
                &star_component::DustTag,
                &star_component::DustFilamentsTag,
                &star_component::H2Tag,
                &star_component::H2CoreTag,
            )>,
        ),
        Added<Transform>,
    >,
    galaxy_setting: Res<galaxy_setting_component::GalaxySettings>,
) {
    for (star, mut transform, mut sprite, star_type) in &mut star_query {
        let pos = calculate_position(
            &galaxy_setting,
            star.a,
            star.b,
            star.theta0,
            star.vel_theta,
            star.tilt_angle,
        );
        transform.translation = pos.extend(0.);

        let pos2 = calculate_position(
            &galaxy_setting,
            star.a + 1000.,
            star.b,
            star.theta0,
            star.vel_theta,
            star.tilt_angle,
        );

        if star_type.0.is_some() {
            sprite.custom_size = Some(Vec2::ONE * star.mag * 4.0);
        }

        if star_type.1.is_some() {
            sprite.custom_size = Some(Vec2::ONE * star.mag * 5.0 * galaxy_setting.dust_render_size);
        }

        if star_type.2.is_some() {
            sprite.custom_size = Some(Vec2::ONE * star.mag * 2.0 * galaxy_setting.dust_render_size);
        }

        if star_type.3.is_some() {
            sprite.custom_size =
                Some(Vec2::ONE * (((1000.0 - Vec2::distance(pos, pos2)) / 10.) - 50.));
        }

        if star_type.4.is_some() {
            sprite.custom_size =
                Some(Vec2::ONE * 0.1 * (((1000.0 - Vec2::distance(pos, pos2)) / 10.) - 50.));
        }
    }
}

const DEG_TO_RAD: f32 = 0.01745329251;
fn calculate_position(
    galaxy_setting: &Res<galaxy_setting_component::GalaxySettings>,
    a: f32,
    b: f32,
    theta0: f32,
    vel_theta: f32,
    tilt_angle: f32,
) -> Vec2 {
    let theta_actual = theta0 + vel_theta;
    let beta = -tilt_angle;
    let alpha = theta_actual * DEG_TO_RAD;
    let cosalpha = alpha.cos();
    let sinalpha = alpha.sin();
    let cosbeta = beta.cos();
    let sinbeta = beta.sin();

    let mut pos = Vec2 {
        x: (a * cosalpha * cosbeta - b * sinalpha * sinbeta),
        y: (a * cosalpha * sinbeta + b * sinalpha * cosbeta),
    };

    if galaxy_setting.pert_amp > 0 && galaxy_setting.pert_n > 0 {
        pos.x += (a / galaxy_setting.pert_amp as f32)
            * (alpha * 2.0 * galaxy_setting.pert_n as f32).sin();
        pos.y += (a / galaxy_setting.pert_amp as f32)
            * (alpha * 2.0 * galaxy_setting.pert_n as f32).cos();
    }
    pos
}
