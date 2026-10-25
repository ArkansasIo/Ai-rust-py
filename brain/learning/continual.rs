use super::{LearningController, LearningEvent};

#[derive(Debug, Default)]
pub struct ContinualLearning {
    pub controller: LearningController,
    pub updates: u64,
}

impl ContinualLearning {
    pub fn new(rate: f32) -> Self {
        Self { controller: LearningController::new(rate), updates: 0 }
    }

    pub fn observe(&mut self, event: LearningEvent) {
        self.controller.observe(event);
        self.updates += 1;
    }
}
