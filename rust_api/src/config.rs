use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub ollama_url: String,
    pub python_ml_url: String,
    pub host: String,
    pub port: u16,
    pub db_max_connections: u32,
    pub default_model: String,
}
impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://ai_user:ai_password@mysql:3306/ai_ml".into()),
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_| "http://ollama:11434".into()),
            python_ml_url: env::var("PYTHON_ML_URL").unwrap_or_else(|_| "http://python-ml:8001".into()),
            host: env::var("RUST_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("RUST_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080),
            db_max_connections: env::var("DB_MAX_CONNECTIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(10),
            default_model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".into()),
        }
    }
}
