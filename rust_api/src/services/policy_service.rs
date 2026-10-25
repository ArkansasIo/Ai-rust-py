use crate::domain::tool::{ToolDefinition, ToolRisk};

pub struct PolicyService;
impl PolicyService {
    pub fn requires_approval(tool: &ToolDefinition) -> bool {
        matches!(tool.risk, ToolRisk::Medium | ToolRisk::High | ToolRisk::Critical)
    }
}
