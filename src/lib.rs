pub mod builder;
pub mod color;
pub mod error;
pub mod pipeline;
pub mod recipes;
pub mod stages;

use image::GenericImageView;
use imgref::ImgVec;
use kolor::{
    ColorSpace, Vec3,
    spaces::{ACES_CG, ENCODED_SRGB},
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::path::Path;

use crate::color::convert_pixels;
pub use crate::error::ProcessingError;

pub struct ProcessingImage {
    pixels: ImgVec<Vec3>,
    space: ColorSpace,
}

impl ProcessingImage {
    pub fn convert(&self, to: ColorSpace) -> Self {
        let pixels = convert_pixels(&self.pixels.buf(), self.space, to);
        Self {
            pixels: ImgVec::new(pixels, self.pixels.width(), self.pixels.height()),
            space: to,
        }
    }

    pub fn iter(&mut self) -> impl ParallelIterator<Item = &mut Vec3> {
        self.pixels.pixels_mut().par_bridge()
    }

    pub fn from_png(path: &Path) -> Result<Self, ProcessingError> {
        Self::from_png_with_colorspace(path, ENCODED_SRGB)
    }

    pub fn from_png_with_colorspace(
        path: &Path,
        source_colorspace: ColorSpace,
    ) -> Result<Self, ProcessingError> {
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

        let pixels_acescg = convert_pixels(&pixels, source_colorspace, ACES_CG);

        Ok(Self {
            pixels: ImgVec::new(pixels_acescg, img.width() as usize, img.height() as usize),
            space: ACES_CG,
        })
    }

    pub fn to_jpg(&self, path: &Path) -> Result<(), ProcessingError> {
        let image_srgb = &self.convert(ENCODED_SRGB);

        let img = image::ImageBuffer::from_fn(
            image_srgb.pixels.width() as u32,
            image_srgb.pixels.height() as u32,
            |x, y| {
                let idx = y as usize * image_srgb.pixels.width() + x as usize;
                let color = image_srgb.pixels.buf().get(idx).unwrap();
                image::Rgb([
                    (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                ])
            },
        );

        img.save(path).map_err(ProcessingError::ImageSave)
    }
}
