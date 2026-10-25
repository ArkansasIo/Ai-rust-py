#[derive(Debug, Clone, Copy)]
pub struct SelfSupervisedLearner {
    pub temperature: f32,
}

impl SelfSupervisedLearner {
    pub fn new(temperature: f32) -> Self { Self { temperature: temperature.max(1e-6) } }

    pub fn prediction_error(&self, prediction: f32, target: f32) -> f32 {
        (prediction - target).abs() / self.temperature
    }
}
