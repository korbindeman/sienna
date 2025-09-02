use crate::{
    pipeline::{Pipeline, ProcessingStage},
    stages::{ColorRichness, ContrastCurve, Exposure, SelectiveColorRichness},
};

pub struct PipelineBuilder {
    stages: Vec<Box<dyn ProcessingStage>>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn exposure(mut self, stops: f32) -> Self {
        self.stages.push(Box::new(Exposure { stops }));
        self
    }

    pub fn richness(mut self, saturation_boost: f32) -> Self {
        self.stages
            .push(Box::new(ColorRichness { saturation_boost }));
        self
    }

    pub fn selective_richness(
        mut self,
        red: f32,
        orange: f32,
        yellow: f32,
        green: f32,
        cyan: f32,
        blue: f32,
        magenta: f32,
    ) -> Self {
        self.stages.push(Box::new(SelectiveColorRichness {
            red_boost: red,
            orange_boost: orange,
            yellow_boost: yellow,
            green_boost: green,
            cyan_boost: cyan,
            blue_boost: blue,
            magenta_boost: magenta,
        }));
        self
    }

    pub fn film_colors(self) -> Self {
        self.selective_richness(
            0.2, // red - slight boost for skin warmth
            0.3, // orange - warm highlights
            0.1, // yellow - subtle warmth
            0.4, // green - rich foliage
            0.2, // cyan - cooler shadows
            0.3, // blue - rich skies
            0.1, // magenta - subtle
        )
    }

    /// Contrast curve
    /// contrast: 1.0 is no change, higher values increase contrast, lower values decrease contrast
    /// pivot: 0.6 is midtones
    pub fn contrast(mut self, contrast: f32, pivot: f32) -> Self {
        self.stages
            .push(Box::new(ContrastCurve { contrast, pivot }));
        self
    }

    pub fn build(self) -> Pipeline {
        Pipeline {
            stages: self.stages,
        }
    }
}
