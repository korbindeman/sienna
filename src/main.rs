use std::path::Path;

use kolor::Vec3;
use sienna::{
    ProcessingImage,
    pipeline::Pipeline,
    stages::{ColorGrade, Exposure, FilmCurve},
};

fn main() {
    let mut image = ProcessingImage::from_png(Path::new("sample.png")).unwrap();

    let pipeline = Pipeline::new()
        .add_stage(Box::new(Exposure { stops: 0.4 }))
        .add_stage(Box::new(ColorGrade {
            shadows: Vec3::new(0.0, 0.02, 0.05),    // Blue shadows
            midtones: Vec3::new(0.05, 0.03, 0.0),   // Warm midtones
            highlights: Vec3::new(0.0, 0.0, -0.02), // Neutral highlights
        }))
        .add_stage(Box::new(FilmCurve { strength: 0.3 }));

    pipeline.process(&mut image);

    image.to_jpg(Path::new("output.jpg")).unwrap();
}
