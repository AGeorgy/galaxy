use bevy::prelude::*;
use bevy_pancam::PanCam;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());

    let sprite_handle = assets.load("particle.png");
    let spacing = 5.0;
    let w = 100;
    let mut sprites = vec![];
    let h = 100;
    for i in 0..w {
        for j in 0..h {
            sprites.push(SpriteBundle {
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
    commands.spawn_batch(sprites);
}
