use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub model: String,
    pub endpoint: String,
    pub max_diff_lines: usize,
    pub stream: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            model: "deepseek-coder".to_string(),
            endpoint: "http://localhost:11434".to_string(),
            max_diff_lines: 400,
            stream: true,
        }
    }
}

fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".smart-commit").join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let path = match config_path() {
        Some(p) => p,
        None => return Ok(Config::default()),
    };

    if !path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

    Ok(config)
}

pub fn merge_with_cli(mut config: Config, cli: &crate::cli::Cli) -> Config {
    if let Some(model) = &cli.model {
        config.model = model.clone();
    }
    if let Some(endpoint) = &cli.endpoint {
        config.endpoint = endpoint.clone();
    }
    if cli.stream {
        config.stream = true;
    }
    config
}
