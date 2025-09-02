use std::f32::consts::PI;

use kolor::{
    Vec3,
    spaces::{ACES_CG, OKLAB},
};
use rayon::prelude::*;

use crate::{ProcessingImage, pipeline::ProcessingStage};

/// Exposure adjustment stage - controls overall image brightness
pub struct Exposure {
    pub stops: f32,
}

impl ProcessingStage for Exposure {
    fn process(&self, image: &mut ProcessingImage) {
        let factor = 2.0f32.powf(self.stops);
        image.iter().for_each(|pixel| {
            pixel.x *= factor;
            pixel.y *= factor;
            pixel.z *= factor;
        });
    }
}

pub struct ContrastCurve {
    pub contrast: f32,
    pub pivot: f32,
}

impl ProcessingStage for ContrastCurve {
    // TODO: skin tone preservation
    fn process(&self, image: &mut ProcessingImage) {
        let mut oklab_image = image.convert(OKLAB);

        oklab_image.iter().for_each(|pixel| {
            // Only apply curve to lightness (L channel)
            pixel.x = apply_contrast_curve(pixel.x, self.contrast, self.pivot);
            // Leave a and b channels unchanged
            // pixel.y and pixel.z stay the same
        });

        *image = oklab_image.convert(ACES_CG);
    }
}

fn apply_contrast_curve(value: f32, contrast: f32, pivot: f32) -> f32 {
    let clamped_value = value.clamp(0.0, 1.0);

    if clamped_value <= pivot {
        // Shadow region (no change from current)
        pivot * (clamped_value / pivot).powf(contrast)
    } else {
        // Highlight region with automatic rolloff
        let highlight_input = (clamped_value - pivot) / (1.0 - pivot);

        // Automatic rolloff: stronger contrast becomes gentler as we approach 1.0
        let rolloff_factor = 1.0 - highlight_input * 0.3; // Gentle automatic rolloff
        let effective_contrast = 1.0 / (contrast * rolloff_factor);

        pivot + (1.0 - pivot) * highlight_input.powf(effective_contrast)
    }
}

pub struct ColorRichness {
    pub saturation_boost: f32, // How much to increase chroma
}

impl ProcessingStage for ColorRichness {
    fn process(&self, image: &mut ProcessingImage) {
        let mut oklab_image = image.convert(OKLAB);

        oklab_image.iter().for_each(|pixel| {
            let lightness = pixel.x;
            let _chroma = (pixel.y * pixel.y + pixel.z * pixel.z).sqrt();

            let midtone_protection = (1.0 - (2.0 * (lightness - 0.6)).abs()).max(0.0);
            let saturation_factor = 1.0 + self.saturation_boost * (1.0 - midtone_protection);

            // Apply saturation boost to a/b channels
            pixel.y *= saturation_factor;
            pixel.z *= saturation_factor;
        });

        *image = oklab_image.convert(ACES_CG);
    }
}

pub struct LegacyColorRichness {
    pub separation_strength: f32,
    pub density_strength: f32,
}

impl ProcessingStage for LegacyColorRichness {
    fn process(&self, image: &mut ProcessingImage) {
        image.iter().for_each(|pixel| {
            let lum = pixel.x * 0.2722 + pixel.y * 0.6741 + pixel.z * 0.0537;

            // Color separation - enhance channel differences
            let avg = (pixel.x + pixel.y + pixel.z) / 3.0;
            let separated = Vec3::new(
                pixel.x + (pixel.x - avg) * self.separation_strength,
                pixel.y + (pixel.y - avg) * self.separation_strength,
                pixel.z + (pixel.z - avg) * self.separation_strength,
            );

            // Tonal density - compress toward rich midtones
            let density_curve = 1.0 - (2.0 * (lum - 0.4)).abs().powf(2.5);
            let dense = separated * (1.0 - 0.15 * density_curve * self.density_strength)
                + Vec3::splat(lum * 0.8) * (0.15 * density_curve * self.density_strength);

            // Preserve luminance while enhancing color
            let new_lum = dense.x * 0.2722 + dense.y * 0.6741 + dense.z * 0.0537;
            let lum_correction = lum / new_lum.max(0.001);

            *pixel = dense * lum_correction;
        });
    }
}

pub struct SelectiveColorRichness {
    pub red_boost: f32,
    pub orange_boost: f32,
    pub yellow_boost: f32,
    pub green_boost: f32,
    pub cyan_boost: f32,
    pub blue_boost: f32,
    pub magenta_boost: f32,
}

impl ProcessingStage for SelectiveColorRichness {
    fn process(&self, image: &mut ProcessingImage) {
        let mut oklab_image = image.convert(OKLAB);

        oklab_image.iter().for_each(|pixel| {
            let chroma = (pixel.y * pixel.y + pixel.z * pixel.z).sqrt();

            if chroma > 0.01 {
                // Only process pixels with actual color
                let hue = pixel.z.atan2(pixel.y); // Hue angle in radians
                let saturation_factor = self.get_saturation_for_hue(hue);

                pixel.y *= saturation_factor;
                pixel.z *= saturation_factor;
            }
        });

        *image = oklab_image.convert(ACES_CG);
    }
}

impl SelectiveColorRichness {
    fn get_saturation_for_hue(&self, hue: f32) -> f32 {
        // Convert hue to 0-2π range
        let normalized_hue = if hue < 0.0 { hue + 2.0 * PI } else { hue };

        // Map hue ranges to boost values (these are approximate OKLAB hue ranges)
        match (normalized_hue / PI * 180.0) as i32 {
            0..=30 => 1.0 + self.red_boost,
            31..=60 => 1.0 + self.orange_boost,
            61..=120 => 1.0 + self.yellow_boost,
            121..=180 => 1.0 + self.green_boost,
            181..=240 => 1.0 + self.cyan_boost,
            241..=300 => 1.0 + self.blue_boost,
            _ => 1.0 + self.magenta_boost,
        }
    }
}
