use crate::domain::workflow::{Workflow, WorkflowStepKind};

pub struct WorkflowService;
impl WorkflowService {
    pub fn validate(workflow: &Workflow) -> Result<(), String> {
        if workflow.name.trim().is_empty() { return Err("workflow name is required".into()); }
        if workflow.steps.is_empty() { return Err("workflow must contain at least one step".into()); }
        if workflow.steps.iter().any(|s| matches!(s.kind, WorkflowStepKind::Tool) && s.config.is_null()) { return Err("tool steps require configuration".into()); }
        Ok(())
    }
}
