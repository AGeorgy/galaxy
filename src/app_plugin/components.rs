use bevy::prelude::*;

pub struct GalaxySettingsResource {
    pub radius: i32,
    pub bulge_radius: f32,
    pub angular_offset: f32,
    pub inner_excentricity: f32,
    pub outter_excentricity: f32,
    pub ellipse_disturbances: i32,
    pub ellipse_disturbances_damping: i32,
    pub count_stars: i32,
    pub count_h2: i32,
    pub has_dark_matter: bool,
    pub base_temp: i32,
    pub dust_render_size: i32,
}

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

#[derive(Component, Debug, Default, Clone)]
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
#[derive(Debug, Default, Clone)]
pub enum StarType {
    #[default]
    Star,
    Dust,
    DustFilaments,
    H2,
}
