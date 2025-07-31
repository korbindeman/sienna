pub mod pipeline;
pub mod stages;

use image::{GenericImageView, ImageError};
use imgref::ImgVec;
use kolor::{
    ColorSpace, Vec3,
    details::conversion::LinearColorConversion,
    spaces::{ACES_CG, LINEAR_SRGB, PRO_PHOTO},
};
use rayon::prelude::*;
use std::path::Path;

pub struct ProcessingImage {
    pixels: ImgVec<Vec3>,
    space: ColorSpace,
}

impl ProcessingImage {
    fn convert(&self, to: ColorSpace) -> Self {
        let pixels = convert_pixels(&self.pixels.buf(), self.space, to);
        Self {
            pixels: ImgVec::new(pixels, self.pixels.width(), self.pixels.height()),
            space: to,
        }
    }
}

pub fn convert_pixels(pixels: &Vec<Vec3>, from: ColorSpace, to: ColorSpace) -> Vec<Vec3> {
    if from == to {
        return pixels.to_vec();
    }

    let converter = LinearColorConversion::new(from, to);
    pixels.par_iter().map(|&p| converter.convert(p)).collect()
}

impl ProcessingImage {
    pub fn from_png(path: &Path) -> Result<Self, ImageError> {
        let img = image::open(path)?;

        let pixels = img
            .pixels()
            .map(|(_x, _y, pixel)| {
                Vec3::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                )
            })
            .collect();

        let pixels_acescg = convert_pixels(&pixels, PRO_PHOTO, ACES_CG);

        Ok(Self {
            pixels: ImgVec::new(pixels_acescg, img.width() as usize, img.height() as usize),
            space: ACES_CG,
        })
    }

    pub fn to_jpg(&self, path: &Path) -> Result<(), ImageError> {
        let image_srgb = &self.convert(LINEAR_SRGB);

        let img = image::ImageBuffer::from_fn(
            image_srgb.pixels.width() as u32,
            image_srgb.pixels.height() as u32,
            |x, y| {
                let idx = y as usize * image_srgb.pixels.width() + x as usize;
                let color = image_srgb.pixels.buf().get(idx).unwrap();
                image::Rgb([
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                ])
            },
        );

        img.save(path)
    }
}
