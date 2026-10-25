//! Computational affective-state models.

#[derive(Debug, Clone, Copy, Default)]
pub struct EmotionalState {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub fear: f32,
    pub joy: f32,
    pub anger: f32,
    pub sadness: f32,
    pub curiosity: f32,
    pub trust: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HomeostaticDrive {
    pub energy: f32,
    pub safety: f32,
    pub social_connection: f32,
    pub novelty: f32,
    pub competence: f32,
}

impl EmotionalState {
    pub fn clamp(&mut self) {
        for value in [
            &mut self.valence, &mut self.arousal, &mut self.dominance,
            &mut self.fear, &mut self.joy, &mut self.anger, &mut self.sadness,
            &mut self.curiosity, &mut self.trust,
        ] {
            *value = value.clamp(0.0, 1.0);
        }
    }
}
