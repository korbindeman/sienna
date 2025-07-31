use crate::ProcessingImage;

/// Trait for image processing stages that can be chained together
pub trait ProcessingStage: Send + Sync {
    fn process(&self, image: &mut ProcessingImage);
}

/// Processing pipeline that executes stages in sequence
pub struct Pipeline {
    pub(crate) stages: Vec<Box<dyn ProcessingStage>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { stages: Vec::new() }
    }

    pub fn add_stage(mut self, stage: Box<dyn ProcessingStage>) -> Self {
        self.stages.push(stage);
        self
    }

    pub fn process(&self, image: &mut ProcessingImage) {
        for stage in &self.stages {
            stage.process(image);
        }
    }
}
