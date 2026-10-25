use crate::models::{AgentResult, AgentSpec};
use crate::ollama::{chat, OllamaError};
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("ollama: {0}")]
    Ollama(#[from] OllamaError),
    #[error("agent not found: {0}")]
    NotFound(String),
}

#[derive(Clone, Default)]
pub struct AgentRegistry {
    agents: HashMap<String, AgentSpec>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        let mut r = Self::default();
        r.register(AgentSpec { name: "general".into(), description: "General purpose assistant".into(), system_prompt: "You are a reliable general-purpose AI agent. Be precise and actionable.".into(), tools: vec![] });
        r.register(AgentSpec { name: "researcher".into(), description: "Research and evidence agent".into(), system_prompt: "You are a research agent. Separate facts, assumptions, uncertainty, and claims requiring verification.".into(), tools: vec!["web.search".into(), "memory.search".into()] });
        r.register(AgentSpec { name: "coder".into(), description: "Software engineering agent".into(), system_prompt: "You are a senior software engineer. Inspect requirements, design maintainable solutions, and produce testable implementation steps.".into(), tools: vec!["filesystem.read".into(), "filesystem.write".into(), "shell.exec".into(), "git.diff".into()] });
        r.register(AgentSpec { name: "reviewer".into(), description: "Code and output review agent".into(), system_prompt: "You are a rigorous reviewer. Find correctness, security, reliability, and maintainability problems before approving work.".into(), tools: vec!["git.diff".into()] });
        r.register(AgentSpec { name: "synthesizer".into(), description: "Result synthesis agent".into(), system_prompt: "You synthesize prior agent outputs into a clear, evidence-aware final answer without inventing facts.".into(), tools: vec![] });
        r
    }

    pub fn register(&mut self, spec: AgentSpec) { self.agents.insert(spec.name.clone(), spec); }
    pub fn get(&self, name: &str) -> Option<&AgentSpec> { self.agents.get(name) }
    pub fn names(&self) -> Vec<String> { self.agents.keys().cloned().collect() }
}

pub async fn execute(client: &Client, base_url: &str, model: &str, registry: &AgentRegistry, agent: &str, task: &str) -> Result<AgentResult, AgentError> {
    let spec = registry.get(agent).ok_or_else(|| AgentError::NotFound(agent.into()))?;
    let prompt = format!("Task:\n{}\n\nAvailable tools: {:?}\n\nReturn a useful result. Do not claim a tool was executed unless it actually was.", task, spec.tools);
    let answer = chat(client, base_url, model, &format!("{}\n{}", spec.system_prompt, prompt)).await?;
    Ok(AgentResult { status: "success".into(), agent: agent.into(), answer, steps: vec![json!({"type":"llm","agent":agent})] })
}
