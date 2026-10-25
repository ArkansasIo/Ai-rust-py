use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal { pub id: String, pub description: String, pub priority: f32 }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExecutiveState { pub active_goals: Vec<Goal>, pub selected_action: Option<String>, pub confidence: f32 }

impl ExecutiveState {
    pub fn select(&mut self, action: impl Into<String>, confidence: f32) { self.selected_action = Some(action.into()); self.confidence = confidence.clamp(0.0, 1.0); }
}
