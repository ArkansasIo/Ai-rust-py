use crate::domain::agent::{AgentConfig, AgentStatus};

pub struct AgentService;
impl AgentService {
    pub fn validate(config: &AgentConfig) -> Result<(), String> {
        if config.profile.name.trim().is_empty() { return Err("agent name is required".into()); }
        if config.max_steps == 0 { return Err("max_steps must be greater than zero".into()); }
        if matches!(config.profile.status, AgentStatus::Disabled) { return Err("agent is disabled".into()); }
        Ok(())
    }
}
