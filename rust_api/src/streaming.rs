use futures_util::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct OllamaChunk {
    message: Option<Message>,
    done: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Message { content: Option<String> }

#[derive(Debug, Error)]
pub enum StreamingError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
}

pub async fn stream_chat<F>(client: &Client, base: &str, model: &str, prompt: &str, mut on_token: F) -> Result<(), StreamingError>
where F: FnMut(String) + Send {
    let url = format!("{}/api/chat", base.trim_end_matches('/'));
    let response = client.post(url)
        .json(&serde_json::json!({"model": model, "stream": true, "messages": [{"role":"user","content":prompt}]}))
        .send().await?;
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    while let Some(chunk) = stream.next().await {
        buffer.push_str(&String::from_utf8_lossy(&chunk?));
        while let Some(pos) = buffer.find('\n') {
            let line = buffer[..pos].trim().to_string();
            buffer.drain(..=pos);
            if line.is_empty() { continue; }
            if let Ok(item) = serde_json::from_str::<OllamaChunk>(&line) {
                if let Some(content) = item.message.and_then(|m| m.content) { on_token(content); }
                if item.done.unwrap_or(false) { return Ok(()); }
            }
        }
    }
    Ok(())
}
