use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub system: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse { pub response: String, pub done: bool }

#[derive(Clone)]
pub struct OllamaCortex {
    client: Client,
    pub endpoint: String,
    pub model: String,
}

impl OllamaCortex {
    pub fn new(endpoint: impl Into<String>, model: impl Into<String>) -> Self {
        Self { client: Client::new(), endpoint: endpoint.into(), model: model.into() }
    }
    pub async fn think(&self, system: Option<&str>, prompt: &str) -> Result<String, reqwest::Error> {
        let req = OllamaRequest { model: self.model.clone(), prompt: prompt.into(), stream: false, system: system.map(str::to_owned) };
        let response = self.client.post(format!("{}/api/generate", self.endpoint.trim_end_matches('/'))).json(&req).send().await?.json::<OllamaResponse>().await?;
        Ok(response.response)
    }
}
