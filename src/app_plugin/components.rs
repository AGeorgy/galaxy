use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use std::f32;

const PC_TO_KM: f32 = 3.08567758129e13;
const SEC_PER_YEAR: f32 = 365.25 * 86400.;
const CONSTANT_OF_GRAVITY: f32 = 6.672e-11;

pub struct GalaxySettingsResource {
    pub radius: f32,
    pub bulge_radius: f32,
    pub far_field_radius: f32,
    pub angular_offset: f32,
    pub inner_excentricity: f32,
    pub outter_excentricity: f32,
    pub ellipse_disturbances: i32,
    pub ellipse_disturbances_damping: i32,
    pub count_stars: i32,
    pub count_h2: i32,
    pub has_dark_matter: bool,
    pub base_temp: f32,
    pub dust_render_size: i32,
    pub seed: u64,
}

impl GalaxySettingsResource {
    pub fn get_excentricity(&self, rad: f32) -> f32 {
        if rad < self.bulge_radius {
            // Core region of the galaxy. Innermost part is round
            // excentricity increasing linear to the border of the core.
            1. + (rad / self.bulge_radius) * (self.inner_excentricity - 1.)
        } else if rad > self.bulge_radius && rad <= self.radius {
            self.inner_excentricity
                + (rad - self.bulge_radius) / (self.radius - self.bulge_radius)
                    * (self.outter_excentricity - self.inner_excentricity)
        } else if rad > self.radius && rad < self.far_field_radius {
            // excentricity is slowly reduced to 1.
            self.outter_excentricity
                + (rad - self.radius) / (self.far_field_radius - self.radius)
                    * (1. - self.outter_excentricity)
        } else {
            1.
        }
    }

    pub fn get_angular_offset(&self, rad: f32) -> f32 {
        rad * self.angular_offset
    }

    pub fn get_orbital_velocity(&self, rad: f32) -> f32 {
        let vel_kms: f32; // velovity in kilometer per seconds
        if self.has_dark_matter {
            vel_kms = GalaxySettingsResource::velocity_with_dark_matter(rad);
        } else {
            vel_kms = GalaxySettingsResource::velocity_without_dark_matter(rad);
        }

        // Calculate velocity in degree per year
        let u = 2.0 * f32::consts::PI * rad * PC_TO_KM;
        let time = u / (vel_kms * SEC_PER_YEAR);

        360.0 / time
    }

    fn velocity_with_dark_matter(r: f32) -> f32 {
        const MZ: f32 = 100.;
        let mass_halo = GalaxySettingsResource::mass_halo(r);
        let mass_disc = GalaxySettingsResource::mass_disc(r);
        let v = 20000.0 * (CONSTANT_OF_GRAVITY * (mass_halo + mass_disc + MZ) / r).sqrt();
        return v;
    }

    fn velocity_without_dark_matter(r: f32) -> f32 {
        const MZ: f32 = 100.;
        20000.0 * (CONSTANT_OF_GRAVITY * (GalaxySettingsResource::mass_disc(r) + MZ) / r).sqrt()
    }

    fn mass_disc(r: f32) -> f32 {
        let d = 2000.; // thickness of the disc
        let rho_so = 1.; // density at the center
        let r_h = 2000.; // radius at which the density has fallen by half
        return rho_so * (-r / r_h).exp() * (r * r) * f32::consts::PI * d;
    }

    fn mass_halo(r: f32) -> f32 {
        let rho_h0 = 0.15;
        let r_c = 2500.;
        return rho_h0 * 1. / (1. + 2_f32.powf(r / r_c))
            * (4. * f32::consts::PI * 3_f32.powf(r) / 3.);
    }
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
    //Dust,
    //DustFilaments,
    //H2,
}
