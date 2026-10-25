use crate::git_tools;
use crate::security::{RiskLevel, SecurityPolicy};
use crate::workspace::WorkspaceManager;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::process::Command;

#[derive(Clone)]
pub struct ToolExecutor {
    pub policy: Arc<SecurityPolicy>,
    pub workspaces: WorkspaceManager,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "tool", rename_all = "snake_case")]
pub enum ToolRequest {
    GitStatus { workspace: String },
    GitDiff { workspace: String },
    GitLog { workspace: String, count: Option<usize> },
    ReadFile { workspace: String, path: String },
    WriteFile { workspace: String, path: String, content: String },
    Shell { workspace: String, command: String },
}

#[derive(Debug, Serialize)]
pub struct ToolResult { pub ok: bool, pub output: String, pub risk: RiskLevel }

impl ToolExecutor {
    pub async fn execute(&self, req: ToolRequest) -> Result<ToolResult, String> {
        match req {
            ToolRequest::GitStatus { workspace } => self.git(PathBuf::from(workspace), "status").await,
            ToolRequest::GitDiff { workspace } => self.git(PathBuf::from(workspace), "diff").await,
            ToolRequest::GitLog { workspace, count } => {
                if !self.policy.allows("git.log", RiskLevel::Read) { return Err("tool denied by policy".into()); }
                let output = git_tools::log(&PathBuf::from(workspace), count.unwrap_or(10)).map_err(|e| e.to_string())?;
                Ok(ToolResult { ok: true, output, risk: RiskLevel::Read })
            }
            ToolRequest::ReadFile { workspace, path } => {
                if !self.policy.allows("filesystem.read", RiskLevel::Read) { return Err("tool denied by policy".into()); }
                let ws = PathBuf::from(workspace);
                let file = self.workspaces.safe_join(&ws, std::path::Path::new(&path)).map_err(|e| e.to_string())?;
                let output = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
                Ok(ToolResult { ok: true, output, risk: RiskLevel::Read })
            }
            ToolRequest::WriteFile { workspace, path, content } => {
                if !self.policy.allows("filesystem.write", RiskLevel::Medium) { return Err("write requires approval".into()); }
                let ws = PathBuf::from(workspace);
                let file = self.workspaces.safe_join(&ws, std::path::Path::new(&path)).map_err(|e| e.to_string())?;
                std::fs::write(file, content).map_err(|e| e.to_string())?;
                Ok(ToolResult { ok: true, output: "file written".into(), risk: RiskLevel::Medium })
            }
            ToolRequest::Shell { .. } => Err("shell execution is disabled by default".into()),
        }
    }

    async fn git(&self, workspace: PathBuf, kind: &str) -> Result<ToolResult, String> {
        if !self.policy.allows(&format!("git.{}", kind), RiskLevel::Read) { return Err("tool denied by policy".into()); }
        let output = match kind {
            "status" => git_tools::status(&workspace),
            "diff" => git_tools::diff(&workspace),
            _ => unreachable!(),
        }.map_err(|e| e.to_string())?;
        Ok(ToolResult { ok: true, output, risk: RiskLevel::Read })
    }
}
