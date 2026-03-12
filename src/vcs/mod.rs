pub mod git;
pub mod svn;

use anyhow::{bail, Result};
use std::path::Path;

pub trait Vcs {
    fn get_diff(&self) -> Result<String>;
    fn commit(&self, message: &str) -> Result<()>;
    fn name(&self) -> &str;
}

pub fn detect_vcs(override_vcs: Option<&str>) -> Result<Box<dyn Vcs>> {
    if let Some(vcs_name) = override_vcs {
        return match vcs_name.to_lowercase().as_str() {
            "git" => Ok(Box::new(git::GitVcs::new())),
            "svn" => Ok(Box::new(svn::SvnVcs::new())),
            other => bail!("Unknown VCS: {}. Supported: git, svn", other),
        };
    }

    let cwd = std::env::current_dir()?;

    if Path::new(".git").exists() || find_git_root(&cwd) {
        return Ok(Box::new(git::GitVcs::new()));
    }

    if Path::new(".svn").exists() {
        return Ok(Box::new(svn::SvnVcs::new()));
    }

    bail!("Could not detect VCS. Are you inside a git or svn repository?")
}

fn find_git_root(start: &Path) -> bool {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".git").exists() {
            return true;
        }
        if !current.pop() {
            return false;
        }
    }
}
