use image::ImageError;
use std::fmt;

#[derive(Debug)]
pub enum ProcessingError {
    ImageLoad(ImageError),
    ImageSave(ImageError),
    InvalidColorSpace,
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::ImageLoad(e) => write!(f, "Failed to load image: {}", e),
            ProcessingError::ImageSave(e) => write!(f, "Failed to save image: {}", e),
            ProcessingError::InvalidColorSpace => write!(f, "Invalid color space conversion"),
        }
    }
}

impl std::error::Error for ProcessingError {}

impl From<ImageError> for ProcessingError {
    fn from(error: ImageError) -> Self {
        ProcessingError::ImageLoad(error)
    }
}