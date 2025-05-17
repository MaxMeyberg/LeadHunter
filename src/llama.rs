use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{json, Value};

/// Ask your local Ollama model (non-streaming) and return the full answer.
pub async fn generate_from_llama(system_prompt: &str, user_prompt: &str) -> Result<String> {
    let client = Client::new();

    let body = json!({
        "model": "MaziyarPanahi/Llama-3.2-1B-Instruct-GGUF",
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user",   "content": user_prompt }
        ],
        "temperature": 0.7,
        "max_tokens": -1,
        "stream": false
    });

    // adjust port/endpoint if needed (11434/api/chat or 1234/v1/chat/completions)
    let resp = client
        .post("http://localhost:1234/v1/chat/completions")
        .json(&body)
        .send()
        .await
        .context("Failed to send request to Ollama")?
        .error_for_status()
        .context("Non-2xx response from Ollama")?;

    let json: Value = resp
        .json()
        .await
        .context("Failed to parse JSON from Ollama")?;

    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    Ok(content)
}