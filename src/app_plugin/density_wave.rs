use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DensityWave {
    pub min: f32,
    pub max: f32,
    pub steps: i32,
    pub i0: f32,
    pub k: f32,
    pub a: f32,
    pub bulge_radius: f32,

    pub m1: Vec<f32>,
    pub y1: Vec<f32>,
    pub x1: Vec<f32>,

    pub m2: Vec<f32>,
    pub y2: Vec<f32>,
    pub x2: Vec<f32>,
}

impl DensityWave {
    pub fn val_from_prob(&self, val: f32) -> f32 {
        let h = 1.0 / (self.y2.len() - 1) as f32;
        let i = (val / h).floor();
        let remainder = val - i * h;

        self.y2[i as usize] + self.m2[i as usize] * remainder
    }

    pub fn build(&mut self) {
        let mut h = (self.max - self.min) / self.steps as f32;
        let mut y = 0.;

        self.x1.clear();
        self.y1.clear();
        self.x2.clear();
        self.y2.clear();
        self.m1.clear();
        self.m2.clear();

        // Simpson rule for integration of the distribution function
        self.y1.push(0.0);
        self.x1.push(0.0);
        for i in (0..self.steps).step_by(2) {
            let x = h * (i + 2) as f32;
            y += h / 3.
                * (self.intensity(self.min + i as f32 * h)
                    + 4. * self.intensity(self.min + (i + 1) as f32 * h)
                    + self.intensity(self.min + (i + 2) as f32 * h));

            self.m1.push((y - self.y1.last().unwrap()) / (2. * h));
            self.x1.push(x);
            self.y1.push(y);
        }
        self.m1.push(0.0);

        // normieren
        for i in 0..self.y1.len() {
            let y1_last = self.y1.last().cloned().unwrap();
            self.y1[i] /= y1_last;
            self.m1[i] /= y1_last;
        }

        self.x2.push(0.0);
        self.y2.push(0.0);

        h = 1. / self.steps as f32;
        let mut k = 0;
        for i in 1..self.steps {
            let p = i as f32 * h;

            while self.y1[k + 1] <= p {
                k += 1;
            }

            y = self.x1[k] + (p - self.y1[k]) / self.m1[k];

            self.m2.push((y - self.y2.last().unwrap()) / h);
            self.x2.push(p);
            self.y2.push(y);
        }
        self.m2.push(0.0);
    }

    fn intensity(&self, x: f32) -> f32 {
        if x < self.bulge_radius {
            self.intensity_bulge(x, self.i0, self.k)
        } else {
            let i0 = self.intensity_bulge(self.bulge_radius, self.i0, self.k);
            self.intensity_disc(x - self.bulge_radius, i0, self.a)
        }
    }

    fn intensity_bulge(&self, r: f32, i0: f32, k: f32) -> f32 {
        i0 * (-k * r.powf(0.25)).exp()
    }

    fn intensity_disc(&self, r: f32, i0: f32, a: f32) -> f32 {
        i0 * (-r / a).exp()
    }
}
