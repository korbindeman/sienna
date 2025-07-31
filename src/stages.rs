use kolor::Vec3;
use rayon::prelude::*;

use crate::{ProcessingImage, pipeline::ProcessingStage};

/// Exposure adjustment stage - controls overall image brightness
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

/// Film-like S-curve stage - adds cinematic contrast curve
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

/// Selective color grading stage - applies different colors to shadows, midtones, and highlights
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

// Shadow crushing with lifted blacks
pub struct FilmBlacks {
    pub crush_point: f32, // 0.0-0.3
    pub lift_amount: f32, // 0.0-0.05
}

impl ProcessingStage for FilmBlacks {
    fn process(&self, image: &mut ProcessingImage) {
        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
            // Crush shadows
            pixel.x = remap_shadows(pixel.x, self.crush_point, self.lift_amount);
            pixel.y = remap_shadows(pixel.y, self.crush_point, self.lift_amount);
            pixel.z = remap_shadows(pixel.z, self.crush_point, self.lift_amount);
        });
    }
}

fn remap_shadows(value: f32, crush: f32, lift: f32) -> f32 {
    if value < crush {
        lift + (value / crush) * (crush - lift)
    } else {
        value
    }
}

// S-curve contrast with adjustable pivot
pub struct ContrastCurve {
    pub contrast: f32,
    pub pivot: f32,
}

impl ProcessingStage for ContrastCurve {
    fn process(&self, image: &mut ProcessingImage) {
        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
            pixel.x = apply_contrast_curve(pixel.x, self.contrast, self.pivot);
            pixel.y = apply_contrast_curve(pixel.y, self.contrast, self.pivot);
            pixel.z = apply_contrast_curve(pixel.z, self.contrast, self.pivot);
        });
    }
}

fn apply_contrast_curve(value: f32, contrast: f32, pivot: f32) -> f32 {
    let normalized = (value - pivot) / (1.0 - pivot);
    let curved = normalized / (1.0 + contrast * normalized.abs()).powf(1.0 / contrast);
    curved * (1.0 - pivot) + pivot
}

// Split-tone grading
pub struct SplitTone {
    pub shadow_hue: f32,
    pub shadow_saturation: f32,
    pub highlight_hue: f32,
    pub highlight_saturation: f32,
}

impl ProcessingStage for SplitTone {
    fn process(&self, image: &mut ProcessingImage) {
        let shadow_color = hsl_to_rgb(self.shadow_hue, self.shadow_saturation, 0.5);
        let highlight_color = hsl_to_rgb(self.highlight_hue, self.highlight_saturation, 0.5);

        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
            let lum = pixel.x * 0.2722 + pixel.y * 0.6741 + pixel.z * 0.0537;

            // Blend colors based on luminance
            let shadow_weight = (1.0 - lum * 2.0).max(0.0);
            let highlight_weight = ((lum - 0.5) * 2.0).max(0.0);

            *pixel = *pixel
                + (shadow_color - Vec3::splat(0.5)) * shadow_weight * 0.2
                + (highlight_color - Vec3::splat(0.5)) * highlight_weight * 0.2;
        });
    }
}

// Selective saturation by luminance
pub struct LuminanceSaturation {
    pub shadow_sat: f32,
    pub midtone_sat: f32,
    pub highlight_sat: f32,
}

impl ProcessingStage for LuminanceSaturation {
    fn process(&self, image: &mut ProcessingImage) {
        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
            let lum = pixel.x * 0.2722 + pixel.y * 0.6741 + pixel.z * 0.0537;

            // Calculate weights
            let shadow_weight = (1.0 - lum * 3.0).max(0.0);
            let highlight_weight = ((lum - 0.7) * 3.3).max(0.0).min(1.0);
            let midtone_weight = 1.0 - shadow_weight - highlight_weight;

            // Weighted saturation multiplier
            let sat_mult = shadow_weight * self.shadow_sat
                + midtone_weight * self.midtone_sat
                + highlight_weight * self.highlight_sat;

            // Apply saturation
            let desaturated = Vec3::splat(lum);
            *pixel = desaturated + (*pixel - desaturated) * sat_mult;
        });
    }
}

// RGB to HSL conversion
fn _rgb_to_hsl(pixel: Vec3) -> (f32, f32, f32) {
    let max = pixel.x.max(pixel.y).max(pixel.z);
    let min = pixel.x.min(pixel.y).min(pixel.z);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if max == pixel.x {
        ((pixel.y - pixel.z) / delta + if pixel.y < pixel.z { 6.0 } else { 0.0 }) / 6.0
    } else if max == pixel.y {
        ((pixel.z - pixel.x) / delta + 2.0) / 6.0
    } else {
        ((pixel.x - pixel.y) / delta + 4.0) / 6.0
    };

    (h * 360.0, s, l)
}

// HSL to RGB conversion
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Vec3 {
    if s == 0.0 {
        return Vec3::splat(l);
    }

    let h = h / 360.0;
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    Vec3::new(
        hue_to_rgb(p, q, h + 1.0 / 3.0),
        hue_to_rgb(p, q, h),
        hue_to_rgb(p, q, h - 1.0 / 3.0),
    )
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

pub struct ColorRichness {
    pub separation_strength: f32,
    pub density_strength: f32,
}

impl ProcessingStage for ColorRichness {
    fn process(&self, image: &mut ProcessingImage) {
        image.pixels.pixels_mut().par_bridge().for_each(|pixel| {
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
