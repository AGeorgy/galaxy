use bevy::prelude::*;
use std::f32;

const PC_TO_KM: f32 = 3.08567758129e13;
const SEC_PER_YEAR: f32 = 365.25 * 86400.;
const CONSTANT_OF_GRAVITY: f32 = 6.672e-11;

#[derive(Resource, Default)]
pub struct GalaxySettings {
    pub radius: f32,
    pub bulge_radius: f32,
    pub far_field_radius: f32,
    pub angular_offset: f32,
    pub inner_excentricity: f32,
    pub outter_excentricity: f32,
    pub ellipse_disturbances: i32,
    pub ellipse_disturbances_damping: i32,
    pub count_stars: usize,
    pub count_dusts: usize,
    pub count_dusts_filaments: usize,
    pub count_h2: i32,
    pub count_h2_core: i32,
    pub has_dark_matter: bool,
    pub base_temp: f32,
    pub dust_render_size: f32,
    pub pert_n: i32,
    pub pert_amp: i32,
    pub seed: u64,
}

impl GalaxySettings {
    // pub fn get_count_all_objects(&self) -> usize {
    //     self.count_stars
    // }

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
            vel_kms = GalaxySettings::velocity_with_dark_matter(rad);
        } else {
            vel_kms = GalaxySettings::velocity_without_dark_matter(rad);
        }

        // Calculate velocity in degree per year
        let u = 2.0 * f32::consts::PI * rad * PC_TO_KM;
        let time = u / (vel_kms * SEC_PER_YEAR);

        360.0 / time
    }

    fn velocity_with_dark_matter(r: f32) -> f32 {
        const MZ: f32 = 100.;
        let mass_halo = GalaxySettings::mass_halo(r);
        let mass_disc = GalaxySettings::mass_disc(r);
        let v = 20000.0 * (CONSTANT_OF_GRAVITY * (mass_halo + mass_disc + MZ) / r).sqrt();
        return v;
    }

    fn velocity_without_dark_matter(r: f32) -> f32 {
        const MZ: f32 = 100.;
        20000.0 * (CONSTANT_OF_GRAVITY * (GalaxySettings::mass_disc(r) + MZ) / r).sqrt()
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
        return rho_h0 * 1. / (1. + (r / r_c).powf(2.)) * (4. * f32::consts::PI * r.powf(3.) / 3.);
    }
}
