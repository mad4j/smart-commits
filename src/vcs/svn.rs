use super::Vcs;
use anyhow::{bail, Context, Result};
use std::process::Command;

pub struct SvnVcs;

impl SvnVcs {
    pub fn new() -> Self {
        SvnVcs
    }
}

impl Vcs for SvnVcs {
    fn name(&self) -> &str {
        "svn"
    }

    fn get_diff(&self) -> Result<String> {
        let diff_output = Command::new("svn")
            .arg("diff")
            .output()
            .context("Failed to run svn diff")?;

        if !diff_output.status.success() {
            bail!(
                "svn diff failed: {}",
                String::from_utf8_lossy(&diff_output.stderr)
            );
        }

        let status_output = Command::new("svn")
            .arg("status")
            .output()
            .context("Failed to run svn status")?;

        if !status_output.status.success() {
            bail!(
                "svn status failed: {}",
                String::from_utf8_lossy(&status_output.stderr)
            );
        }

        let diff = String::from_utf8_lossy(&diff_output.stdout).to_string();
        let status = String::from_utf8_lossy(&status_output.stdout).to_string();

        if diff.trim().is_empty() && status.trim().is_empty() {
            bail!("No SVN changes detected.");
        }

        let mut combined = String::new();
        if !status.trim().is_empty() {
            combined.push_str("SVN Status:\n");
            combined.push_str(&status);
            combined.push('\n');
        }
        combined.push_str(&diff);

        Ok(combined)
    }

    fn commit(&self, message: &str) -> Result<()> {
        let output = Command::new("svn")
            .args(["commit", "-m", message])
            .output()
            .context("Failed to run svn commit")?;

        if !output.status.success() {
            bail!(
                "svn commit failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}
