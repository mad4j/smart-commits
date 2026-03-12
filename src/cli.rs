use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "smart-commit", about = "AI-powered commit message generator")]
pub struct Cli {
    /// Preview commit message without committing
    #[arg(long)]
    pub dry_run: bool,

    /// Automatically perform commit with generated message
    #[arg(long)]
    pub apply: bool,

    /// Enable streaming output
    #[arg(long)]
    pub stream: bool,

    /// Only generate summary line (no bullet points)
    #[arg(long)]
    pub short: bool,

    /// Override VCS detection (git or svn)
    #[arg(long)]
    pub vcs: Option<String>,

    /// Override model name
    #[arg(long)]
    pub model: Option<String>,

    /// Override LLM endpoint URL
    #[arg(long)]
    pub endpoint: Option<String>,
}
