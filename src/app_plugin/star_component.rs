use bevy::{prelude::*, reflect::FromReflect, render::texture::DEFAULT_IMAGE_HANDLE};
use std::f32;

#[derive(Bundle, Clone)]
pub struct StarSpriteBundle {
    pub star: Star,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for StarSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            star: Default::default(),
        }
    }
}

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Star {
    pub theta0: f32,         // initial angular position on the ellipse
    pub vel_theta: f32,      // angular velocity
    pub tilt_angle: f32,     // tilt angle of the ellipse
    pub a: f32,              // semi-minor axes
    pub b: f32,              // semi-major axes
    pub temp: f32,           // star temperature
    pub mag: f32,            // brightness;
    pub star_type: StarType, // Type 0:star, 1:dust, 2 and 3: h2 regions
}

#[derive(Debug, Default, Clone, Reflect, FromReflect)]
#[reflect_value()]
pub enum StarType {
    #[default]
    Star,
    //Dust,
    //DustFilaments,
    //H2,
}

impl StarType {}
