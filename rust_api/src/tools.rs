use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("tool is not registered: {0}")]
    NotFound(String),
    #[error("tool execution is not permitted: {0}")]
    Denied(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel { Read, Low, Medium, High, Critical }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub risk: RiskLevel,
    pub requires_approval: bool,
}

#[derive(Clone, Default)]
pub struct ToolRegistry { tools: Vec<ToolSpec> }

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: vec![
            ToolSpec { name: "memory.search".into(), description: "Search durable AI memory".into(), risk: RiskLevel::Read, requires_approval: false },
            ToolSpec { name: "filesystem.read".into(), description: "Read an approved workspace file".into(), risk: RiskLevel::Read, requires_approval: false },
            ToolSpec { name: "filesystem.write".into(), description: "Write an approved workspace file".into(), risk: RiskLevel::Medium, requires_approval: true },
            ToolSpec { name: "shell.exec".into(), description: "Execute a command in an isolated workspace".into(), risk: RiskLevel::High, requires_approval: true },
            ToolSpec { name: "git.diff".into(), description: "Inspect repository changes".into(), risk: RiskLevel::Read, requires_approval: false },
            ToolSpec { name: "git.commit".into(), description: "Create a repository commit".into(), risk: RiskLevel::High, requires_approval: true },
        ] }
    }
    pub fn list(&self) -> &[ToolSpec] { &self.tools }
    pub fn get(&self, name: &str) -> Result<&ToolSpec, ToolError> { self.tools.iter().find(|t| t.name == name).ok_or_else(|| ToolError::NotFound(name.into())) }
    pub fn authorize(&self, name: &str, approved: bool) -> Result<Value, ToolError> {
        let tool = self.get(name)?;
        if tool.requires_approval && !approved { return Err(ToolError::Denied(name.into())); }
        Ok(serde_json::json!({"tool":tool.name,"authorized":true,"risk":tool.risk}))
    }
}
