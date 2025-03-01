//! Selectors for anyframe-rs
//!
//! Selectors are interactive filtering tools like peco, percol, fzf, and fzf-tmux.

use crate::{error, Result};
use std::io::Write;
use std::process::Command;

/// Trait for selectors
pub trait Selector {
    /// Run the selector with the given input
    fn select(&self, input: &str, query: Option<&str>) -> Result<String>;

    /// Get the name of the selector
    fn name(&self) -> &str;
}

/// Peco selector
pub struct Peco {
    path: String,
}

impl Peco {
    /// Create a new Peco selector
    #[must_use]
    pub fn new(path: Option<String>) -> Self {
        Self {
            path: path.unwrap_or_else(|| "peco".to_string()),
        }
    }
}

impl Selector for Peco {
    fn select(&self, input: &str, query: Option<&str>) -> Result<String> {
        let mut cmd = Command::new(&self.path);

        if let Some(q) = query {
            cmd.arg("--query").arg(q);
        }

        // Create a child process for peco
        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(error::AnyframeError::IoError)?;

        // Write input to peco's stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input.as_bytes())
                .map_err(error::AnyframeError::IoError)?;
        } else {
            return Err(error::AnyframeError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to open stdin for peco",
            )));
        }

        // Wait for peco to finish and get output
        let output = child
            .wait_with_output()
            .map_err(error::AnyframeError::IoError)?;

        if !output.status.success() {
            // Check if the error is due to user cancellation (peco returns 1 when cancelled)
            if output.status.code() == Some(1) && output.stdout.is_empty() {
                return Err(error::AnyframeError::SelectorNotFound(
                    "Selection cancelled by user".to_string(),
                ));
            }

            return Err(error::AnyframeError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "peco command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            )));
        }

        // Convert output to string and trim whitespace
        let selected = String::from_utf8(output.stdout)
            .map_err(|e| {
                error::AnyframeError::IoError(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid UTF-8 in peco output: {}", e),
                ))
            })?
            .trim()
            .to_string();

        if selected.is_empty() {
            return Err(error::AnyframeError::SelectorNotFound(
                "No item selected".to_string(),
            ));
        }

        Ok(selected)
    }

    fn name(&self) -> &'static str {
        "peco"
    }
}

// Similar implementations for Percol, Fzf, and FzfTmux to be added
