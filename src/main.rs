use std::path::Path;

use sienna::{ProcessingError, ProcessingImage, builder::PipelineBuilder, pipeline::Pipeline};

fn main() -> Result<(), ProcessingError> {
    let filename = "test";
    let mut image = ProcessingImage::from_png(Path::new(format!("{}.png", filename).as_str()))?;

    let pipeline = create_leica_style_pipeline();

    pipeline.process(&mut image);

    image.to_jpg(Path::new(format!("{}.jpg", filename).as_str()))?;
    Ok(())
}

pub fn create_leica_style_pipeline() -> Pipeline {
    PipelineBuilder::new()
        // .exposure(0.1)
        .film_blacks(0.5, 0.01)
        // .color_grade(
        //     Vec3::new(-0.01, 0.005, 0.02),
        //     Vec3::new(0.015, 0.01, -0.005),
        //     Vec3::new(0.0, -0.005, -0.01),
        // )
        // .split_tone(220.0, 0.12, 35.0, 0.08)
        // .luminance_saturation(0.85, 1.15, 0.65)
        // .richness(0.3, 0.8)
        // .film_curve(0.2)
        .build()
}
