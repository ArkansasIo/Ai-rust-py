use std::{path::Path, process::Command};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("git command failed: {0}")]
    Command(String),
    #[error("git I/O error: {0}")]
    Io(#[from] std::io::Error),
}

fn run_git(dir: &Path, args: &[&str]) -> Result<String, GitError> {
    let output = Command::new("git").args(args).current_dir(dir).output()?;
    if !output.status.success() {
        return Err(GitError::Command(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn status(dir: &Path) -> Result<String, GitError> { run_git(dir, &["status", "--short"]) }
pub fn diff(dir: &Path) -> Result<String, GitError> { run_git(dir, &["diff", "--no-ext-diff"]) }
pub fn log(dir: &Path, count: usize) -> Result<String, GitError> {
    run_git(dir, &["log", "--oneline", &format!("-{}", count)])
}
