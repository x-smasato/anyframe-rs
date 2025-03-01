//! Selectors for anyframe-rs
//!
//! Selectors are interactive filtering tools like peco, percol, fzf, and fzf-tmux.

use crate::Result;
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
    pub fn new(path: Option<String>) -> Self {
        Self {
            path: path.unwrap_or_else(|| "peco".to_string()),
        }
    }
}

impl Selector for Peco {
    fn select(&self, _input: &str, query: Option<&str>) -> Result<String> {
        let mut cmd = Command::new(&self.path);

        if let Some(q) = query {
            cmd.arg("--query").arg(q);
        }

        // Implementation details to be added

        Ok("Selected item".to_string()) // Placeholder
    }

    fn name(&self) -> &str {
        "peco"
    }
}

// Similar implementations for Percol, Fzf, and FzfTmux to be added
