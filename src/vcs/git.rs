use super::Vcs;
use anyhow::{bail, Context, Result};
use std::process::Command;

pub struct GitVcs;

impl GitVcs {
    pub fn new() -> Self {
        GitVcs
    }
}

impl Vcs for GitVcs {
    fn name(&self) -> &str {
        "git"
    }

    fn get_diff(&self) -> Result<String> {
        // First try staged diff
        let staged = Command::new("git")
            .args(["diff", "--staged"])
            .output()
            .context("Failed to run git diff --staged")?;

        if !staged.status.success() {
            bail!(
                "git diff --staged failed: {}",
                String::from_utf8_lossy(&staged.stderr)
            );
        }

        let staged_diff = String::from_utf8_lossy(&staged.stdout).to_string();

        if !staged_diff.trim().is_empty() {
            return Ok(staged_diff);
        }

        // Fall back to unstaged diff
        let unstaged = Command::new("git")
            .args(["diff", "HEAD"])
            .output()
            .context("Failed to run git diff HEAD")?;

        if !unstaged.status.success() {
            bail!(
                "git diff HEAD failed: {}",
                String::from_utf8_lossy(&unstaged.stderr)
            );
        }

        let unstaged_diff = String::from_utf8_lossy(&unstaged.stdout).to_string();

        if unstaged_diff.trim().is_empty() {
            bail!("No changes detected. Stage your changes with 'git add' first.");
        }

        Ok(unstaged_diff)
    }

    fn commit(&self, message: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .context("Failed to run git commit")?;

        if !output.status.success() {
            bail!(
                "git commit failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}
