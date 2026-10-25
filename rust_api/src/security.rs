use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub allow_shell: bool,
    pub allow_network: bool,
    pub require_approval_for_writes: bool,
    pub max_tool_calls: u32,
}

impl Default for SecurityPolicy {
    fn default() -> Self { Self { allow_shell: false, allow_network: false, require_approval_for_writes: true, max_tool_calls: 32 } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub action: String,
    pub resource: String,
    pub reason: String,
    pub approved: bool,
}

impl SecurityPolicy {
    pub fn can_execute(&self, action: &str, approved: bool) -> bool {
        if action == "shell.exec" && !self.allow_shell { return false; }
        if action.starts_with("network.") && !self.allow_network { return false; }
        if self.require_approval_for_writes && (action.contains("write") || action.contains("commit")) && !approved { return false; }
        true
    }
}
