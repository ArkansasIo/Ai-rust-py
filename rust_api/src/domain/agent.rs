use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentClass { Assistant, Researcher, Coder, Analyst, Planner, Executor, Custom }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus { Created, Ready, Running, Paused, Failed, Disabled }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability { pub name: String, pub enabled: bool, pub risk_level: u8 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub id: Uuid,
    pub name: String,
    pub class: AgentClass,
    pub status: AgentStatus,
    pub system_prompt: String,
    pub model: String,
    pub temperature: f32,
    pub capabilities: Vec<AgentCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig { pub profile: AgentProfile, pub max_steps: u32, pub timeout_seconds: u64 }
