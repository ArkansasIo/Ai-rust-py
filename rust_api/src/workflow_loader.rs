use serde_json::Value;
use std::fs;

pub fn load_json(path: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}
