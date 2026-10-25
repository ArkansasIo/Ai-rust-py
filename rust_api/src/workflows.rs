use crate::models::WorkflowDefinition;
use serde_json::Value;
use std::fs;

pub fn load_workflow(path: &str) -> Result<WorkflowDefinition, Box<dyn std::error::Error + Send + Sync>> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn validate_workflow(workflow: &WorkflowDefinition) -> Result<Value, String> {
    if workflow.name.trim().is_empty() { return Err("workflow name is required".into()); }
    if workflow.nodes.is_empty() { return Err("workflow must contain at least one node".into()); }
    let node_ids: std::collections::HashSet<_> = workflow.nodes.iter().map(|n| n.id.as_str()).collect();
    for edge in &workflow.edges {
        if !node_ids.contains(edge.from.as_str()) || !node_ids.contains(edge.to.as_str()) {
            return Err(format!("invalid edge {} -> {}", edge.from, edge.to));
        }
    }
    Ok(serde_json::json!({"valid":true,"nodes":workflow.nodes.len(),"edges":workflow.edges.len()}))
}
