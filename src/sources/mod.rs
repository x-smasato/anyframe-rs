//! Sources for anyframe-rs
//!
//! Sources provide data to be filtered, such as command history, directories, processes, etc.

use crate::{error, Result};
use std::process::Command;

/// Trait for sources
pub trait Source {
    /// Get the data from the source
    fn get_data(&self) -> Result<String>;

    /// Get the name of the source
    fn name(&self) -> &str;
}

/// History source
pub struct History;

impl Source for History {
    fn get_data(&self) -> Result<String> {
        // Get command history using zsh
        let history_output = Command::new("zsh")
            .arg("-c")
            .arg("history -n -r 1 | awk '!a[$0]++'")
            .output()
            .map_err(|e| {
                error::AnyframeError::SourceError(format!("Failed to execute zsh: {}", e))
            })?;

        if !history_output.status.success() {
            return Err(error::AnyframeError::SourceError(format!(
                "zsh command failed: {}",
                String::from_utf8_lossy(&history_output.stderr)
            )));
        }

        let history_str = String::from_utf8(history_output.stdout).map_err(|e| {
            error::AnyframeError::SourceError(format!("Invalid UTF-8 in history output: {}", e))
        })?;

        Ok(history_str)
    }

    fn name(&self) -> &str {
        "history"
    }
}

/// Directory source
pub struct Directory;

impl Source for Directory {
    fn get_data(&self) -> Result<String> {
        // Get current directory path
        let current_dir = std::env::current_dir().map_err(|e| {
            error::AnyframeError::SourceError(format!("Failed to get current directory: {}", e))
        })?;

        // Read directory entries
        let entries = std::fs::read_dir(&current_dir).map_err(|e| {
            error::AnyframeError::SourceError(format!("Failed to read directory: {}", e))
        })?;

        // Collect file names
        let mut file_list = String::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                error::AnyframeError::SourceError(format!("Failed to read directory entry: {}", e))
            })?;

            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            file_list.push_str(&file_name_str);
            file_list.push('\n');
        }

        Ok(file_list)
    }

    fn name(&self) -> &str {
        "directory"
    }
}

/// Process source
pub struct Process;

impl Source for Process {
    fn get_data(&self) -> Result<String> {
        // Get process list using ps command
        let username = std::env::var("USER").map_err(|e| {
            error::AnyframeError::SourceError(format!(
                "Failed to get USER environment variable: {}",
                e
            ))
        })?;

        let ps_output = Command::new("ps")
            .arg("-u")
            .arg(&username)
            .arg("-o")
            .arg("pid,stat,%cpu,%mem,cputime,command")
            .output()
            .map_err(|e| {
                error::AnyframeError::SourceError(format!("Failed to execute ps command: {}", e))
            })?;

        if !ps_output.status.success() {
            return Err(error::AnyframeError::SourceError(format!(
                "ps command failed: {}",
                String::from_utf8_lossy(&ps_output.stderr)
            )));
        }

        let ps_str = String::from_utf8(ps_output.stdout).map_err(|e| {
            error::AnyframeError::SourceError(format!("Invalid UTF-8 in ps output: {}", e))
        })?;

        Ok(ps_str)
    }

    fn name(&self) -> &str {
        "process"
    }
}

/// Ghq repository source
pub struct GhqRepository;

impl Source for GhqRepository {
    fn get_data(&self) -> Result<String> {
        // Get ghq repository list using ghq command
        let ghq_output = Command::new("ghq")
            .arg("list")
            .arg("--full-path")
            .output()
            .map_err(|e| {
                error::AnyframeError::SourceError(format!("Failed to execute ghq command: {}", e))
            })?;

        if !ghq_output.status.success() {
            return Err(error::AnyframeError::SourceError(format!(
                "ghq command failed: {}",
                String::from_utf8_lossy(&ghq_output.stderr)
            )));
        }

        let ghq_str = String::from_utf8(ghq_output.stdout).map_err(|e| {
            error::AnyframeError::SourceError(format!("Invalid UTF-8 in ghq output: {}", e))
        })?;

        Ok(ghq_str)
    }

    fn name(&self) -> &str {
        "ghq-repository"
    }
}

// Similar implementations for other sources to be added
