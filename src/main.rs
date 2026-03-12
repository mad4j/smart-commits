mod cli;
mod config;
mod diff;
mod llm;
mod output;
mod prompt;
mod vcs;

use anyhow::{Context, Result};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    // Load and merge config
    let config = config::load_config().context("Failed to load config")?;
    let config = config::merge_with_cli(config, &cli);

    // Detect VCS
    let vcs = vcs::detect_vcs(cli.vcs.as_deref())
        .context("Failed to detect or initialize VCS")?;

    println!("Using VCS: {}", vcs.name());

    // Get diff
    let raw_diff = vcs.get_diff().context("Failed to get diff")?;

    // Preprocess diff
    let preprocessed = diff::preprocess_diff(&raw_diff, config.max_diff_lines);

    // Build prompt
    let prompt_text = prompt::build_prompt(&preprocessed, cli.short);

    println!("Generating commit message using model: {}", config.model);

    // Call LLM
    let message = if config.stream {
        println!("\nStreaming response:");
        llm::stream::generate_streaming(&config.endpoint, &config.model, &prompt_text)
            .await
            .context("Failed to generate streaming response")?
    } else {
        llm::client::generate(&config.endpoint, &config.model, &prompt_text)
            .await
            .context("Failed to generate response")?
    };

    // Handle output
    output::handle_output(&message, cli.dry_run, cli.apply, vcs.as_ref())?;

    Ok(())
}
