//! Software cognitive pipeline inspired by distributed brain functions.
//! These are engineering abstractions, not one-to-one biological models.

#[derive(Debug, Clone, Copy, Default)]
pub struct EmotionState { pub valence: f32, pub arousal: f32, pub threat: f32, pub reward: f32 }

#[derive(Debug, Clone, Default)]
pub struct WorkingMemory { pub capacity: usize, pub items: Vec<f32> }
impl WorkingMemory { pub fn new(capacity: usize) -> Self { Self { capacity, items: Vec::with_capacity(capacity) } } pub fn push(&mut self, value: f32) { if self.capacity == 0 { return; } if self.items.len() >= self.capacity { self.items.remove(0); } self.items.push(value); } }

#[derive(Debug, Clone, Default)]
pub struct LongTermMemory { pub traces: Vec<Vec<f32>> }
impl LongTermMemory { pub fn encode(&mut self, trace: Vec<f32>) { self.traces.push(trace); } }

#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutiveState { pub attention: f32, pub inhibition: f32, pub planning: f32, pub confidence: f32 }

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionDecision { pub action_id: u32, pub utility: f32 }

#[derive(Debug, Clone, Default)]
pub struct CognitiveState { pub emotion: EmotionState, pub working_memory: WorkingMemory, pub long_term_memory: LongTermMemory, pub executive: ExecutiveState, pub last_action: Option<ActionDecision> }

impl CognitiveState {
    pub fn new() -> Self { Self { working_memory: WorkingMemory::new(7), ..Default::default() } }
    pub fn perceive(&mut self, salience: f32) { self.executive.attention = salience.clamp(0.0, 1.0); self.working_memory.push(salience); }
    pub fn evaluate(&mut self, threat: f32, reward: f32) { self.emotion.threat = threat.clamp(0.0, 1.0); self.emotion.reward = reward.clamp(0.0, 1.0); self.emotion.arousal = ((threat + reward) * 0.5).clamp(0.0, 1.0); }
    pub fn decide(&mut self, action_id: u32, utility: f32) -> ActionDecision { let d = ActionDecision { action_id, utility }; self.last_action = Some(d); d }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_memory_has_fixed_capacity() {
        let mut memory = WorkingMemory::new(2);
        memory.push(1.0);
        memory.push(2.0);
        memory.push(3.0);
        assert_eq!(memory.items, vec![2.0, 3.0]);
    }

    #[test]
    fn zero_capacity_memory_discards_values() {
        let mut memory = WorkingMemory::new(0);
        memory.push(1.0);
        assert!(memory.items.is_empty());
    }

    #[test]
    fn perception_and_evaluation_are_clamped() {
        let mut state = CognitiveState::new();
        state.perceive(2.0);
        state.evaluate(-1.0, 2.0);
        assert_eq!(state.executive.attention, 1.0);
        assert_eq!(state.emotion.threat, 0.0);
        assert_eq!(state.emotion.reward, 1.0);
        assert_eq!(state.emotion.arousal, 0.5);
    }

    #[test]
    fn decision_is_recorded() {
        let mut state = CognitiveState::new();
        let decision = state.decide(42, 0.75);
        assert_eq!(decision.action_id, 42);
        assert_eq!(state.last_action, Some(decision));
    }
}
