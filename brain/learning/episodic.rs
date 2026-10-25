use super::LearningEvent;

#[derive(Debug, Default)]
pub struct EpisodicLearner {
    episodes: Vec<Vec<LearningEvent>>,
}

impl EpisodicLearner {
    pub fn begin(&mut self) { self.episodes.push(Vec::new()); }

    pub fn record(&mut self, event: LearningEvent) {
        if self.episodes.is_empty() { self.begin(); }
        if let Some(ep) = self.episodes.last_mut() { ep.push(event); }
    }

    pub fn episodes(&self) -> &[Vec<LearningEvent>] { &self.episodes }
}
