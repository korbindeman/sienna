use kolor::Vec3;

use crate::{
    pipeline::{Pipeline, ProcessingStage},
    stages::{
        ColorGrade, ColorRichness, ContrastCurve, Exposure, FilmBlacks, FilmCurve,
        LuminanceSaturation, SplitTone,
    },
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

    pub fn richness(mut self, separation: f32, density: f32) -> Self {
        self.stages.push(Box::new(ColorRichness {
            separation_strength: separation,
            density_strength: density,
        }));
        self
    }

    pub fn film_blacks(mut self, crush: f32, lift: f32) -> Self {
        self.stages.push(Box::new(FilmBlacks {
            crush_point: crush,
            lift_amount: lift,
        }));
        self
    }

    pub fn contrast(mut self, strength: f32, pivot: f32) -> Self {
        self.stages.push(Box::new(ContrastCurve {
            contrast: strength,
            pivot,
        }));
        self
    }

    pub fn split_tone(
        mut self,
        shadow_hue: f32,
        shadow_sat: f32,
        highlight_hue: f32,
        highlight_sat: f32,
    ) -> Self {
        self.stages.push(Box::new(SplitTone {
            shadow_hue,
            shadow_saturation: shadow_sat,
            highlight_hue,
            highlight_saturation: highlight_sat,
        }));
        self
    }

    pub fn color_grade(mut self, shadows: Vec3, midtones: Vec3, highlights: Vec3) -> Self {
        self.stages.push(Box::new(ColorGrade {
            shadows,
            midtones,
            highlights,
        }));
        self
    }

    pub fn luminance_saturation(mut self, shadow: f32, midtone: f32, highlight: f32) -> Self {
        self.stages.push(Box::new(LuminanceSaturation {
            shadow_sat: shadow,
            midtone_sat: midtone,
            highlight_sat: highlight,
        }));
        self
    }

    pub fn film_curve(mut self, strength: f32) -> Self {
        self.stages.push(Box::new(FilmCurve { strength }));
        self
    }

    pub fn build(self) -> Pipeline {
        Pipeline {
            stages: self.stages,
        }
    }
}
