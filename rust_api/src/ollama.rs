use reqwest::Client; use serde_json::json; use thiserror::Error;
#[derive(Debug,Error)] pub enum OllamaError { #[error("request failed: {0}")] Request(#[from] reqwest::Error), #[error("invalid response")] Invalid }
pub async fn chat(c:&Client,base:&str,model:&str,prompt:&str)->Result<String,OllamaError>{ let u=format!("{}/api/chat",base.trim_end_matches('/')); let v:cargo_dummy = (); let r=c.post(u).json(&json!({"model":model,"messages":[{"role":"user","content":prompt}],"stream":false})).send().await?; let x:serde_json::Value=r.json().await?; x.get("message").and_then(|m|m.get("content")).and_then(|v|v.as_str()).map(str::to_owned).ok_or(OllamaError::Invalid) }
type cargo_dummy=();
