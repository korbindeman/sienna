use kolor::{
    ColorSpace, Vec3,
    details::conversion::{ColorConversion, LinearColorConversion},
};
use rayon::prelude::*;

pub fn convert_pixels(pixels: &Vec<Vec3>, from: ColorSpace, to: ColorSpace) -> Vec<Vec3> {
    if from == to {
        return pixels.to_vec();
    }

    if from.is_linear() && to.is_linear() {
        let converter = LinearColorConversion::new(from, to);
        pixels.par_iter().map(|&p| converter.convert(p)).collect()
    } else {
        let converter = ColorConversion::new(from, to);
        pixels.par_iter().map(|&p| converter.convert(p)).collect()
    }
}
