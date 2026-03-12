use anyhow::{bail, Context, Result};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct StreamChunk {
    response: Option<String>,
    done: Option<bool>,
    error: Option<String>,
}

pub async fn generate_streaming(endpoint: &str, model: &str, prompt: &str) -> Result<String> {
    let client = Client::new();
    let url = format!("{}/api/generate", endpoint.trim_end_matches('/'));

    let request_body = GenerateRequest {
        model,
        prompt,
        stream: true,
    };

    let resp = client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .with_context(|| format!("Failed to connect to LLM at {}", url))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("LLM API error {}: {}", status, body);
    }

    let mut stream = resp.bytes_stream();
    let mut full_response = String::new();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.context("Error reading stream chunk")?;
        let text = String::from_utf8_lossy(&bytes);
        buffer.push_str(&text);

        // Process complete lines from buffer
        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer = buffer[newline_pos + 1..].to_string();

            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<StreamChunk>(&line) {
                Ok(chunk) => {
                    if let Some(err) = chunk.error {
                        bail!("LLM streaming error: {}", err);
                    }
                    if let Some(token) = chunk.response {
                        print!("{}", token);
                        use std::io::Write;
                        let _ = std::io::stdout().flush();
                        full_response.push_str(&token);
                    }
                    if chunk.done.unwrap_or(false) {
                        println!();
                        return Ok(full_response);
                    }
                }
                Err(_) => {
                    // Skip malformed lines (e.g. keep-alive or unexpected chunks)
                }
            }
        }
    }

    Ok(full_response)
}
