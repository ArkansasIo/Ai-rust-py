use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveEvent {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub payload: serde_json::Value,
    pub priority: u8,
    pub timestamp_ms: u128,
}

impl CognitiveEvent {
    pub fn new(kind: impl Into<String>, source: impl Into<String>, payload: serde_json::Value) -> Self {
        let timestamp_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
        Self { id: format!("evt-{timestamp_ms}"), kind: kind.into(), source: source.into(), payload, priority: 5, timestamp_ms }
    }
}
