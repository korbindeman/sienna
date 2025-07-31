use kolor::Vec3;
use rayon::prelude::*;

use crate::{ProcessingImage, pipeline::ProcessingStage};

// Exposure adjustment
pub struct Exposure {
    pub stops: f32,
}

impl ProcessingStage for Exposure {
    fn process(&self, image: &mut ProcessingImage) {
        let factor = 2.0f32.powf(self.stops);
        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
            pixel.x *= factor;
            pixel.y *= factor;
            pixel.z *= factor;
        });
    }
}

// Film-like S-curve
pub struct FilmCurve {
    pub strength: f32,
}

impl ProcessingStage for FilmCurve {
    fn process(&self, image: &mut ProcessingImage) {
        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
            pixel.x = apply_curve(pixel.x, self.strength);
            pixel.y = apply_curve(pixel.y, self.strength);
            pixel.z = apply_curve(pixel.z, self.strength);
        });
    }
}

fn apply_curve(x: f32, strength: f32) -> f32 {
    // Lifted blacks, compressed highlights
    let lifted = x + 0.003 * strength;
    lifted / (1.0 + lifted * strength * 0.8)
}

// Selective color grading
pub struct ColorGrade {
    pub shadows: Vec3,
    pub midtones: Vec3,
    pub highlights: Vec3,
}

impl ProcessingStage for ColorGrade {
    fn process(&self, image: &mut ProcessingImage) {
        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
            let lum = pixel.x * 0.2722 + pixel.y * 0.6741 + pixel.z * 0.0537;

            // Three-way color correction
            let shadow_weight = (1.0 - lum).max(0.0);
            let highlight_weight = lum.max(0.0);
            let midtone_weight = 1.0 - (2.0 * (lum - 0.5)).abs();

            *pixel = *pixel
                + self.shadows * shadow_weight * 0.1
                + self.midtones * midtone_weight * 0.05
                + self.highlights * highlight_weight * 0.05;
        });
    }
}
