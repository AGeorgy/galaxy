use bevy::prelude::*;
use std::f32;

#[derive(Bundle, Clone)]
pub struct StarSpriteBundle {
    pub star: Star,
    pub view: SpriteBundle,
}

impl Default for StarSpriteBundle {
    fn default() -> Self {
        Self {
            star: Default::default(),
            view: Default::default(),
        }
    }
}

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Star {
    pub theta0: f32,     // initial angular position on the ellipse
    pub vel_theta: f32,  // angular velocity
    pub tilt_angle: f32, // tilt angle of the ellipse
    pub a: f32,          // semi-minor axes
    pub b: f32,          // semi-major axes
    pub temp: f32,       // star temperature
    pub mag: f32,        // brightness;
}

#[derive(Component, Deref)]
pub struct Alpha(pub f32);

// Type 0:star, 1:dust, 2 and 3: h2 regions
#[derive(Component)]
pub struct StarTag;
#[derive(Component)]
pub struct DustTag;
#[derive(Component)]
pub struct DustFilamentsTag;
#[derive(Component)]
pub struct H2Tag;
#[derive(Component)]
pub struct H2CoreTag;
