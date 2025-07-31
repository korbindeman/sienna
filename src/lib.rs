//! Sienna - A high-performance image processing library
//!
//! This library provides tools for advanced image processing with color-accurate
//! workflows using ACES color space for professional image manipulation.

pub mod builder;
pub mod color;
pub mod error;
pub mod pipeline;
pub mod stages;

use image::GenericImageView;
use imgref::ImgVec;
use kolor::{
    ColorSpace, Vec3,
    spaces::{ACES_CG, LINEAR_SRGB, PRO_PHOTO},
};
use std::path::Path;

use crate::color::convert_pixels;
pub use crate::error::ProcessingError;

/// Core image structure for processing operations
///
/// Stores image data in a linear color space with associated metadata.
/// All processing operations work in ACES-CG color space for accuracy.
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

    pub fn from_png(path: &Path) -> Result<Self, ProcessingError> {
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

    pub fn to_jpg(&self, path: &Path) -> Result<(), ProcessingError> {
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

        img.save(path).map_err(ProcessingError::ImageSave)
    }
}
