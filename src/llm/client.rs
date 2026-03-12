use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct GenerateResponse {
    response: Option<String>,
    error: Option<String>,
}

pub async fn generate(endpoint: &str, model: &str, prompt: &str) -> Result<String> {
    let client = Client::new();
    let url = format!("{}/api/generate", endpoint.trim_end_matches('/'));

    let request_body = GenerateRequest {
        model,
        prompt,
        stream: false,
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

    let generate_resp: GenerateResponse = resp
        .json()
        .await
        .context("Failed to parse LLM response")?;

    if let Some(err) = generate_resp.error {
        bail!("LLM returned error: {}", err);
    }

    generate_resp
        .response
        .ok_or_else(|| anyhow::anyhow!("LLM response missing 'response' field"))
}
