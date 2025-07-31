use kolor::{ColorSpace, Vec3, details::conversion::LinearColorConversion};
use rayon::prelude::*;

/// Convert pixels between color spaces using parallel processing
pub fn convert_pixels(pixels: &Vec<Vec3>, from: ColorSpace, to: ColorSpace) -> Vec<Vec3> {
    if from == to {
        return pixels.to_vec();
    }

    let converter = LinearColorConversion::new(from, to);
    pixels.par_iter().map(|&p| converter.convert(p)).collect()
}