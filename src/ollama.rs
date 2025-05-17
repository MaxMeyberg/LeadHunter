use reqwest::Client;
use serde_json::Value;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // build our JSON payload
    let body = serde_json::json!({
        "model": "MaziyarPanahi/Llama-3.2-1B-Instruct-GGUF",
        "messages": [
            { "role": "system", "content": "Be brief." },
            { "role": "user",   "content": "Introduce yourself." }
        ],
        "temperature": 0.7,
        "max_tokens": -1,
        "stream": true
    });

    // send request and get a byte stream
    let resp_stream = client
        .post("http://localhost:1234/v1/chat/completions")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?
        .bytes_stream();

    tokio::pin!(resp_stream);

    let mut buffer = String::new();

    while let Some(chunk) = resp_stream.next().await {
        let bytes = chunk?;
        buffer.push_str(std::str::from_utf8(&bytes)?);

        // extract and process full lines
        while let Some(pos) = buffer.find('\n') {
            // clone the line out so we don't hold a borrow on buffer
            let mut line = buffer[..pos].to_string();
            // remove the line (and the newline) from buffer
            buffer.drain(..=pos);

            line = line.trim_end().to_string();

            if let Some(payload) = line.strip_prefix("data: ") {
                let payload = payload.trim();
                if payload == "[DONE]" {
                    return Ok(());
                }
                if let Ok(json) = serde_json::from_str::<Value>(payload) {
                    if let Some(content) = json
                        .get("choices")
                        .and_then(|c| c.get(0))
                        .and_then(|c0| c0.get("delta"))
                        .and_then(|d| d.get("content"))
                        .and_then(|v| v.as_str())
                    {
                        print!("{}", content);
                    }
                }
            }
        }
    }

    Ok(())
}
