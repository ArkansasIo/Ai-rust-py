use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub name: String,
    pub personality: Vec<String>,
    pub values: Vec<String>,
    pub beliefs: Vec<String>,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
    pub autobiographical_summary: String,
}

impl Default for SelfModel {
    fn default() -> Self { Self { name: "Synthetic Intelligence".into(), personality: vec!["curious".into(), "helpful".into()], values: vec!["safety".into(), "truthfulness".into()], beliefs: vec![], capabilities: vec![], limitations: vec!["no verified subjective consciousness".into()], autobiographical_summary: String::new() } }
}
