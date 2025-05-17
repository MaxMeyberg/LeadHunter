use reqwest::Client;
use serde_json::Value;
use anyhow::{Result, Context};
use futures_util::StreamExt;
use tokio::io::AsyncBufReadExt;
use crate::{ScrapeLinkedInResponse, ImproveEmailResponse};
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};
use futures_util::stream::Stream;

/// Helper: Send streaming request to local LLM and collect the content deltas into a full JSON string.
async fn stream_llm_request(body: Value) -> Result<String> {
    println!("▶️ Starting LLM request with body:\n{}", body);
    let client = Client::new();
    let response = client
        .post("http://localhost:1234/v1/chat/completions")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .context("❌ Failed to send LLM request")?;
    println!("✅ Request sent, status: {}", response.status());

    let mut stream = response.bytes_stream();
    let mut full_content = String::new();
    let mut chunk_count = 0;

    'outer: while let Some(chunk) = stream.next().await {
        chunk_count += 1;
        println!("---\n📦 Received chunk #{}", chunk_count);
        let chunk = match chunk {
            Ok(c) => c,
            Err(e) => {
                println!("⚠️ Error reading chunk: {:?}", e);
                return Err(e.into());
            }
        };
        let chunk_str = match std::str::from_utf8(&chunk) {
            Ok(s) => s,
            Err(e) => {
                println!("⚠️ Invalid UTF-8 in chunk: {:?}", e);
                return Err(e.into());
            }
        };
        println!("📄 Chunk content (len={}):\n{}", chunk_str.len(),
            if chunk_str.len() > 200 { format!("{}... (truncated)", &chunk_str[..200]) } else { chunk_str.to_string() }
        );

        for (i, raw_line) in chunk_str.lines().enumerate() {
            let line = raw_line.trim();
            println!("   ↳ line {}: {:?}", i + 1, line);
            if line.is_empty() {
                println!("     • skipped empty line");
                continue;
            }
            if let Some(payload) = line.strip_prefix("data:") {
                let payload = payload.trim();
                println!("     • SSE payload: {:?}", payload);

                // SSE [DONE]
                if payload == "[DONE]" {
                    println!(
                        "🏁 Received SSE [DONE], returning collected content ({} chars)",
                        full_content.len()
                    );
                    return Ok(full_content);
                }

                // Try to parse the JSON payload
                match serde_json::from_str::<Value>(payload) {
                    Ok(json) => {
                        println!("     • Parsed JSON keys: {:?}", json.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()));
                        // finish_reason?
                        if let Some(reason) = json
                            .get("choices")
                            .and_then(|c| c.get(0))
                            .and_then(|c0| c0.get("finish_reason"))
                            .and_then(|v| v.as_str())
                        {
                            println!("     • finish_reason = {:?}", reason);
                            if reason == "stop" {
                                println!(
                                    "🏁 finish_reason stop, returning content ({} chars)",
                                    full_content.len()
                                );
                                return Ok(full_content);
                            }
                        }
                        // content delta?
                        if let Some(delta) = json
                            .get("choices")
                            .and_then(|c| c.get(0))
                            .and_then(|c0| c0.get("delta"))
                            .and_then(|d| d.get("content"))
                            .and_then(|v| v.as_str())
                        {
                            println!("     • delta.content = {:?}", delta);
                            full_content.push_str(delta);
                            println!("     • full_content now {} chars", full_content.len());
                        } else {
                            println!("     • no delta.content in this JSON chunk");
                        }
                    }
                    Err(e) => {
                        println!("     • ❌ Failed to parse JSON: {:?}\n       payload was: {}", e, payload);
                    }
                }
            } else {
                println!("     • line did not start with 'data:'");
            }
        }
    } // end outer

    println!(
        "🔚 Stream ended after {} chunks, total collected {} chars",
        chunk_count,
        full_content.len()
    );

    let trimmed = full_content.trim();
    println!("✂️ After trimming whitespace: {} chars", trimmed.len());
    if trimmed.is_empty() {
        println!("⚠️ No content received, bailing out");
        anyhow::bail!("No content was received from the stream");
    }

    println!("🔍 Attempting final JSON parse…");
    match serde_json::from_str::<Value>(trimmed) {
        Ok(_) => {
            println!("✅ Final content is valid JSON, returning.");
            Ok(trimmed.to_string())
        }
        Err(e) => {
            println!("❌ Final parse error: {:?}", e);
            println!("Raw trimmed content:\n{}", trimmed);
            anyhow::bail!("Failed to parse LLM response as JSON: {}", e);
        }
    }
}




/// Generates a personalized cold email based on LinkedIn data and a prompt.
pub async fn generate_email(ws_info: Value, prompt: &str) -> Result<ScrapeLinkedInResponse> {
    let full_name = ws_info["fullName"].as_str().unwrap_or("");
    let headline = ws_info["headline"].as_str().unwrap_or("");
    let about = ws_info["about"].as_str().unwrap_or("");
    let email_addr = ws_info["email"].as_str().unwrap_or("").to_string();

    let system_prompt = "You're a skilled B2B copywriter who knows how to write cold emails that actually get replies. Your job is to craft short, thoughtful, and personalized emails for enterprise decision-makers based on their LinkedIn profiles and a quick briefing on the product or service being offered.\n\nAlways start with: Dear [First Name],\nKeep it brief — aim for 4 to 6 sentences total\nMake it personal — use relevant LinkedIn details to show you’ve done your homework\nFocus on real value — how does this offering help solve a challenge or make their work easier, faster, or more effective?\nUse a natural, conversational tone — like it was written by a thoughtful human\nEnd with a light, low-pressure CTA — like asking if they’d be open to a quick call\nAvoid all fluff — skip generic intros like \"Hope you're well,\" marketing buzzwords, or long walls of text\n\n**Output format (JSON only)**:\n{\n  \"email_output\": \"...\",\n  \"analysis_rationale\": [\"...\",\"...\"]\n}";
    let user_content = format!(
        "Here is the LinkedIn profile info:\nName: {}\nHeadline: {}\nAbout: {}\n\nPrompt: {}",
        full_name, headline, about, prompt
    );

    let body = serde_json::json!({
        "model": "MaziyarPanahi/Llama-3.2-1B-Instruct-GGUF",
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_content }
        ],
        "temperature": 0.7,
        "max_tokens": -1,
        "stream": true
    });

    let raw = stream_llm_request(body)
        .await
        .context("LLM stream failed")?;

    // 2) DEBUG: dump it all
    println!("\n🚨 Raw LLM output ({} bytes):\n<<START>>\n{}\n<<END>>\n", raw.len(), raw);

    // 3) trim whitespace
    let trimmed = raw.trim();
    println!("✂️ After trim: {} bytes:\n<<START>>\n{}\n<<END>>\n", trimmed.len(), trimmed);

    // 4) attempt to isolate the JSON (first “{” through last “}”)
    let maybe_json = if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        &trimmed[start..=end]
    } else {
        trimmed
    };
    println!("🔍 Candidate JSON slice ({} bytes):\n<<START>>\n{}\n<<END>>", maybe_json.len(), maybe_json);

    // Balance braces: count “{” vs “}”
    let open = maybe_json.matches('{').count();
    let close = maybe_json.matches('}').count();
    let mut fixed = maybe_json.to_string();
    if open > close {
        let missing = open - close;
        println!("⚠️ JSON has {} more ‘{{’ than ‘}}’. Appending {} closing brace(s).", open - close, missing);
        fixed.push_str(&"}".repeat(missing));
    }

    println!("🛠️ Fixed JSON slice ({} bytes):\n<<START>>\n{}\n<<END>>", fixed.len(), fixed);

    // Now parse the balanced JSON
    let parsed: Value = serde_json::from_str(&fixed)
        .context("Failed to parse JSON from LLM stream content")?;

    // 6) extract as before
    let email_body = parsed["email_output"].as_str().unwrap_or("").to_string();
    let analysis_rationale = parsed["analysis_rationale"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Ok(ScrapeLinkedInResponse { email_address: email_addr, email_body, analysis_rationale })
}

/// Improves an existing cold email based on instructions.
pub async fn improve_email(
    email: &str,
    prompt: &str,
    recipient_name: Option<String>,
) -> Result<ImproveEmailResponse> {
    let name = recipient_name.unwrap_or_else(|| "there".to_string());

    let system_prompt = "You're a skilled B2B copywriter who knows how to improve cold emails to make them more effective. Your job is to refine and enhance an existing email based on specific improvement instructions.\n\nAlways start with: Dear [First Name],\nKeep it brief — aim for 4 to 6 sentences total\nMake it personal and maintain any personalization from the original email\nFocus on real value — how does this offering help solve a challenge or make their work easier, faster, or more effective?\nUse a natural, conversational tone — like it was written by a thoughtful human\nEnd with a light, low-pressure CTA — like asking if they'd be open to a quick call\nAvoid all fluff — skip generic intros like \"Hope you're well,\" marketing buzzwords, or long walls of text\n\n**Output format (JSON only)**:\n{\n  \"email_output\": \"...\",\n  \"improvement_rationale\": [\"...\",\"...\"]\n}";
    let user_content = format!(
        "Here is the original email:\n{}\nThe recipient's name is {}.\nImprovement instructions: {}",
        email, name, prompt
    );

    let body = serde_json::json!({
        "model": "MaziyarPanahi/Llama-3.2-1B-Instruct-GGUF",
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_content }
        ],
        "temperature": 0.7,
        "max_tokens": -1,
        "stream": true
    });

    let raw = stream_llm_request(body).await.context("LLM stream failed")?;
    let parsed: Value = serde_json::from_str(&raw).context("Failed to parse JSON from LLM stream content")?;

    let improved_email = parsed["email_output"].as_str().unwrap_or("").to_string();
    let improvement_rationale = parsed["improvement_rationale"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Ok(ImproveEmailResponse { improved_email, improvement_rationale })
}
