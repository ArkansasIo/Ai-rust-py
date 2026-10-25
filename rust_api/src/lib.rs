pub mod agents;
pub mod api;
pub mod config;
pub mod models;
pub mod ollama;
pub mod runtime;
pub mod security;
pub mod tools;
pub mod workflow;
pub mod workflows;

use sqlx::{MySqlPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub http: reqwest::Client,
    pub config: config::Config,
}

pub type SharedState = Arc<AppState>;
