use std::path::Path;

use sienna::{ProcessingError, ProcessingImage, builder::PipelineBuilder};

fn main() -> Result<(), ProcessingError> {
    let filename = "small";
    let mut image = ProcessingImage::from_png(Path::new(format!("{}.png", filename).as_str()))?;

    let pipeline = create_pipeline().build();

    pipeline.process(&mut image);

    image.to_jpg(Path::new(format!("{}.jpg", filename).as_str()))?;
    Ok(())
}

pub fn create_pipeline() -> PipelineBuilder {
    PipelineBuilder::new()
        .contrast(0.8, 0.65)
        .exposure(0.1)
        // .contrast(1.3, 0.65)
        .selective_richness(
            0.0,  // red
            0.0,  // orange
            -0.3, // yellow
            0.9,  // green
            0.2,  // cyan
            0.4,  // blue
            0.1,  // magenta
        )
}
