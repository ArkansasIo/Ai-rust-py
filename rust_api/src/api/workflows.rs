use axum::{extract::State,http::StatusCode,Json}; use serde::Deserialize; use serde_json::Value; use std::sync::Arc; use crate::{workflow,AppState};
#[derive(Deserialize)] pub struct WorkflowRequest { pub workflow:String, pub task:String, pub model:Option<String> }
pub async fn run_workflow(State(s):State<Arc<AppState>>,Json(r):Json<WorkflowRequest>)->Result<Json<Value>,(StatusCode,String)>{ let v=workflow::execute(&s.http,&s.config.ollama_url,r.model.as_deref().unwrap_or(&s.config.default_model),&r.workflow,&r.task).await.map_err(|e|(StatusCode::BAD_GATEWAY,e.to_string()))?; Ok(Json(v)) }
