use bevy::prelude::*;
use rand::prelude::*;

use super::star_component::Star;
use super::{density_wave, galaxy_setting_component};
use super::{lod_setting_resource, star_component};

pub fn update_stars(
    mut commands: Commands,
    galaxy_setting: Res<galaxy_setting_component::GalaxySettings>,
    density_wave: Res<density_wave::DensityWave>,
    mut stars: Query<Entity, With<star_component::Star>>,
    assets: Res<AssetServer>,
    lod_settings: Res<lod_setting_resource::LodSetting>,
) {
    if !galaxy_setting.is_changed() && !density_wave.is_changed() {
        return;
    }

    if !stars.is_empty() {
        // delete all
        for star_entity in &mut stars {
            commands.entity(star_entity).despawn();
        }
    }

    create_all_objects(
        &mut commands,
        galaxy_setting,
        density_wave,
        assets.load("particle.png"),
        &lod_settings,
    );
}

fn create_all_objects(
    commands: &mut Commands,
    galaxy_setting: Res<galaxy_setting_component::GalaxySettings>,
    density_wave: Res<density_wave::DensityWave>,
    sprite_handle: Handle<Image>,
    lod_settings: &Res<lod_setting_resource::LodSetting>,
) {
    let mut rnd = StdRng::seed_from_u64(galaxy_setting.seed);

    commands.spawn_batch(create_dusts(
        &galaxy_setting,
        &density_wave,
        &sprite_handle,
        &mut rnd,
        lod_settings.is_other_visibile,
    ));

    commands.spawn_batch(create_dusts_filaments(
        &galaxy_setting,
        &sprite_handle,
        &mut rnd,
        lod_settings.is_other_visibile,
    ));

    commands.spawn_batch(create_h2(
        &galaxy_setting,
        &sprite_handle,
        &mut rnd,
        lod_settings.is_other_visibile,
    ));
    commands.spawn_batch(create_h2_core(
        &galaxy_setting,
        &sprite_handle,
        &mut rnd,
        lod_settings.is_other_visibile,
    ));

    commands.spawn_batch(create_stars(
        &galaxy_setting,
        &density_wave,
        &sprite_handle,
        &mut rnd,
        lod_settings.is_stars_visibile,
    ));
}

fn create_h2(
    galaxy_setting: &Res<galaxy_setting_component::GalaxySettings>,
    sprite_handle: &Handle<Image>,
    rnd: &mut StdRng,
    is_visibile: bool,
) -> Vec<(star_component::H2Tag, star_component::StarSpriteBundle)> {
    let mut stars: Vec<(star_component::H2Tag, star_component::StarSpriteBundle)> = vec![];
    for _i in 0..galaxy_setting.count_h2 {
        let x: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
        let y: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
        let rad = f32::sqrt(x * x + y * y);

        let temp = 6000. + (6000. * rnd.gen::<f32>() - 3000.);
        let mag = 0.1 + 0.05 * rnd.gen::<f32>();
        let b = rad * galaxy_setting.get_excentricity(rad);
        let star_sprite = star_component::StarSpriteBundle {
            star: Star {
                theta0: 360.0 * rnd.gen::<f32>(),
                vel_theta: galaxy_setting.get_orbital_velocity((rad + b) / 2.),
                tilt_angle: galaxy_setting.get_angular_offset(rad),
                a: rad,
                b: b,
                temp: temp,
                mag: mag,
            },
            view: SpriteBundle {
                texture: sprite_handle.clone(),
                visibility: Visibility {
                    is_visible: is_visibile,
                },
                ..default()
            },
        };
        stars.push((star_component::H2Tag, star_sprite));
    }

    stars
}

fn create_h2_core(
    galaxy_setting: &Res<galaxy_setting_component::GalaxySettings>,
    sprite_handle: &Handle<Image>,
    rnd: &mut StdRng,
    is_visibile: bool,
) -> Vec<(star_component::H2CoreTag, star_component::StarSpriteBundle)> {
    let mut stars: Vec<(star_component::H2CoreTag, star_component::StarSpriteBundle)> = vec![];
    for _i in 0..galaxy_setting.count_h2 {
        let x: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
        let y: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
        let rad = f32::sqrt(x * x + y * y);

        let temp = 6000. + (6000. * rnd.gen::<f32>() - 3000.);
        let mag = 0.1 + 0.05 * rnd.gen::<f32>();
        let b = rad * galaxy_setting.get_excentricity(rad);
        let star_sprite = star_component::StarSpriteBundle {
            star: Star {
                theta0: 360.0 * rnd.gen::<f32>(),
                vel_theta: galaxy_setting.get_orbital_velocity((rad + b) / 2.),
                tilt_angle: galaxy_setting.get_angular_offset(rad),
                a: rad,
                b: b,
                temp: temp,
                mag: mag,
            },
            view: SpriteBundle {
                texture: sprite_handle.clone(),
                visibility: Visibility {
                    is_visible: is_visibile,
                },
                ..default()
            },
        };
        stars.push((star_component::H2CoreTag, star_sprite));
    }

    stars
}

fn create_stars(
    galaxy_setting: &Res<galaxy_setting_component::GalaxySettings>,
    density_wave: &Res<density_wave::DensityWave>,
    sprite_handle: &Handle<Image>,
    rnd: &mut StdRng,
    is_visibile: bool,
) -> Vec<(star_component::StarTag, star_component::StarSpriteBundle)> {
    let mut temp = 6000.;
    let mut mag = 1.0;
    let mut stars: Vec<(star_component::StarTag, star_component::StarSpriteBundle)> = vec![];
    // First star ist the black hole at the centre
    let star = star_component::StarSpriteBundle {
        star: Star {
            theta0: 0.,
            vel_theta: 0.,
            tilt_angle: 0.,
            a: 0.,
            b: 0.,
            temp: temp,
            mag: mag,
        },
        view: SpriteBundle {
            visibility: Visibility {
                is_visible: is_visibile,
            },
            texture: sprite_handle.clone(),
            ..default()
        },
    };
    stars.push((star_component::StarTag, star));

    // Initialize stars
    for i in 1..galaxy_setting.count_stars {
        let rad: f32 = density_wave.val_from_prob(rnd.gen());
        temp = (4000. * rnd.gen::<f32>() - 2000.) + 6000.;
        mag = 0.1 + 0.4 * rnd.gen::<f32>();
        let mut star_sprite = star_component::StarSpriteBundle {
            star: Star {
                theta0: 360.0 * rnd.gen::<f32>(),
                vel_theta: galaxy_setting.get_orbital_velocity(rad),
                tilt_angle: galaxy_setting.get_angular_offset(rad),
                a: rad,
                b: rad * galaxy_setting.get_excentricity(rad),
                temp: temp,
                mag: mag,
            },
            view: SpriteBundle {
                texture: sprite_handle.clone(),
                visibility: Visibility {
                    is_visible: is_visibile,
                },
                ..default()
            },
        };

        // Make a small portion of the stars brighter
        if i < galaxy_setting.count_stars / 60 {
            star_sprite.star.mag = 1_f32.min(star_sprite.star.mag + 0.1 + rnd.gen::<f32>() * 0.4);
        }

        stars.push((star_component::StarTag, star_sprite));
    }
    stars
}

fn create_dusts(
    galaxy_setting: &Res<galaxy_setting_component::GalaxySettings>,
    density_wave: &Res<density_wave::DensityWave>,
    sprite_handle: &Handle<Image>,
    rnd: &mut StdRng,
    is_visibile: bool,
) -> Vec<(star_component::DustTag, star_component::StarSpriteBundle)> {
    let mag = 0.02 + 0.15 * rnd.gen::<f32>();
    let mut rad: f32;
    let mut stars: Vec<(star_component::DustTag, star_component::StarSpriteBundle)> = vec![];

    // Initialize dust
    for i in 0..galaxy_setting.count_dusts {
        if i % 2 == 0 {
            rad = density_wave.val_from_prob(rnd.gen());
        } else {
            let x: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
            let y: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
            rad = f32::sqrt(x * x + y * y);
        }

        let temp = galaxy_setting.base_temp + rad / 4.5;
        let b = rad * galaxy_setting.get_excentricity(rad);
        let mut star_sprite = star_component::StarSpriteBundle {
            star: Star {
                theta0: 360.0 * rnd.gen::<f32>(),
                vel_theta: galaxy_setting.get_orbital_velocity((rad + b) / 2.),
                tilt_angle: galaxy_setting.get_angular_offset(rad),
                a: rad,
                b: rad * galaxy_setting.get_excentricity(rad),
                temp,
                mag,
            },
            view: SpriteBundle {
                texture: sprite_handle.clone(),
                visibility: Visibility {
                    is_visible: is_visibile,
                },
                ..default()
            },
        };

        // Make a small portion of the stars brighter
        if i < galaxy_setting.count_stars / 60 {
            star_sprite.star.mag = 1_f32.min(star_sprite.star.mag + 0.1 + rnd.gen::<f32>() * 0.4);
        }

        stars.push((star_component::DustTag, star_sprite));
    }
    stars
}

fn create_dusts_filaments(
    galaxy_setting: &Res<galaxy_setting_component::GalaxySettings>,
    sprite_handle: &Handle<Image>,
    rnd: &mut StdRng,
    is_visibile: bool,
) -> Vec<(
    star_component::DustFilamentsTag,
    star_component::StarSpriteBundle,
)> {
    const FACTOR: usize = 100;
    let mut stars: Vec<(
        star_component::DustFilamentsTag,
        star_component::StarSpriteBundle,
    )> = vec![];
    for _i in 0..galaxy_setting.count_dusts_filaments / FACTOR {
        {
            let x: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
            let y: f32 = 2. * galaxy_setting.radius * rnd.gen::<f32>() - galaxy_setting.radius;
            let mut rad = f32::sqrt(x * x + y * y);

            let theta = 360.0 * rnd.gen::<f32>();
            let mag = 0.1 + 0.05 * rnd.gen::<f32>();
            let temp = galaxy_setting.base_temp + rad / 4.5 - 1000.;
            let b = rad * galaxy_setting.get_excentricity(rad);
            let num = (FACTOR as f32 * rnd.gen::<f32>()) as usize;
            for _j in 0..num {
                rad = rad + 200. - 400. * rnd.gen::<f32>();
                let star_sprite = star_component::StarSpriteBundle {
                    star: Star {
                        theta0: theta + 10. - 20. * rnd.gen::<f32>(),
                        vel_theta: galaxy_setting.get_orbital_velocity((rad + b) / 2.),
                        tilt_angle: galaxy_setting.get_angular_offset(rad),
                        a: rad,
                        b: rad * galaxy_setting.get_excentricity(rad),
                        temp,
                        mag: mag + 0.025 * rnd.gen::<f32>(),
                    },
                    view: SpriteBundle {
                        texture: sprite_handle.clone(),
                        visibility: Visibility {
                            is_visible: is_visibile,
                        },
                        ..default()
                    },
                };
                stars.push((star_component::DustFilamentsTag, star_sprite));
            }
        }
    }
    stars
}
