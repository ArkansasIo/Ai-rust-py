use axum::{extract::State,Json}; use serde_json::{json,Value}; use std::sync::Arc; use crate::AppState;
pub async fn models(State(s):State<Arc<AppState>>)->Json<Value>{ let u=format!("{}/api/tags",s.config.ollama_url.trim_end_matches('/')); match s.http.get(u).send().await { Ok(r)=>Json(r.json().await.unwrap_or_else(|_|json!({"models":[]}))), Err(e)=>Json(json!({"error":e.to_string()})) } }
