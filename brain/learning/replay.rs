use super::LearningEvent;

#[derive(Debug)]
pub struct ReplayBuffer {
    capacity: usize,
    events: Vec<LearningEvent>,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { capacity: capacity.max(1), events: Vec::new() }
    }

    pub fn push(&mut self, event: LearningEvent) {
        if self.events.len() >= self.capacity { self.events.remove(0); }
        self.events.push(event);
    }

    pub fn len(&self) -> usize { self.events.len() }
    pub fn is_empty(&self) -> bool { self.events.is_empty() }
    pub fn samples(&self) -> &[LearningEvent] { &self.events }
}
