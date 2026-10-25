use crate::models::{AgentResult, AgentTask};
use crate::agents::{execute, AgentRegistry};
use reqwest::Client;

pub struct AgentRuntime {
    pub registry: AgentRegistry,
}

impl AgentRuntime {
    pub fn new() -> Self { Self { registry: AgentRegistry::new() } }

    pub async fn run(&self, client: &Client, ollama_url: &str, default_model: &str, request: AgentTask) -> Result<AgentResult, crate::agents::AgentError> {
        execute(client, ollama_url, request.model.as_deref().unwrap_or(default_model), &self.registry, &request.agent, &request.task).await
    }
}
