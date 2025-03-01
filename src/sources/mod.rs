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

/// Git branch source
pub struct GitBranch {
    include_current_branch: bool,
    show_remote_branches: bool,
    show_all_branches: bool,
}

impl GitBranch {
    /// Create a new GitBranch source
    pub fn new(
        include_current_branch: bool,
        show_remote_branches: bool,
        show_all_branches: bool,
    ) -> Self {
        Self {
            include_current_branch,
            show_remote_branches,
            show_all_branches,
        }
    }
}

impl Source for GitBranch {
    fn get_data(&self) -> Result<String> {
        // Build git branch command options
        let mut args = vec!["branch", "--list", "-v"];

        if self.show_all_branches {
            args.push("-a");
        } else if self.show_remote_branches {
            args.push("-r");
        }

        // Execute git branch command
        let branch_output = Command::new("git").args(&args).output().map_err(|e| {
            error::AnyframeError::SourceError(format!("Failed to execute git: {}", e))
        })?;

        if !branch_output.status.success() {
            return Err(error::AnyframeError::SourceError(format!(
                "git command failed: {}",
                String::from_utf8_lossy(&branch_output.stderr)
            )));
        }

        let branch_str = String::from_utf8(branch_output.stdout).map_err(|e| {
            error::AnyframeError::SourceError(format!("Invalid UTF-8 in branch output: {}", e))
        })?;

        // Process output based on whether to include current branch
        let processed_output = if self.include_current_branch {
            // If including current branch, remove '*' marker
            branch_str
                .lines()
                .map(|line| {
                    if line.starts_with('*') {
                        // Remove '*' from lines starting with '*'
                        line.replacen('*', " ", 1).trim_start().to_string()
                    } else {
                        // Remove leading whitespace from other lines
                        line.trim_start().to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            // If not including current branch, exclude lines starting with '*'
            branch_str
                .lines()
                .filter(|line| !line.starts_with('*'))
                .map(|line| line.trim_start().to_string())
                .collect::<Vec<String>>()
                .join("\n")
        };

        Ok(processed_output)
    }

    fn name(&self) -> &str {
        "git-branch"
    }
}

/// Git status source
pub struct GitStatus {
    pattern: Option<String>,
}

impl GitStatus {
    /// Create a new GitStatus source
    pub fn new(pattern: Option<String>) -> Self {
        Self { pattern }
    }
}

impl Source for GitStatus {
    fn get_data(&self) -> Result<String> {
        // Get relative path from git root directory
        let base_path_output = Command::new("git")
            .args(["rev-parse", "--show-cdup"])
            .output()
            .map_err(|e| {
                error::AnyframeError::SourceError(format!("Failed to execute git: {}", e))
            })?;

        if !base_path_output.status.success() {
            return Err(error::AnyframeError::SourceError(format!(
                "git command failed: {}",
                String::from_utf8_lossy(&base_path_output.stderr)
            )));
        }

        let base_path = String::from_utf8(base_path_output.stdout).map_err(|e| {
            error::AnyframeError::SourceError(format!("Invalid UTF-8 in base path output: {}", e))
        })?;

        let base_path = base_path.trim();

        // Execute git status command
        let status_output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .map_err(|e| {
                error::AnyframeError::SourceError(format!("Failed to execute git: {}", e))
            })?;

        if !status_output.status.success() {
            return Err(error::AnyframeError::SourceError(format!(
                "git command failed: {}",
                String::from_utf8_lossy(&status_output.stderr)
            )));
        }

        let status_str = String::from_utf8(status_output.stdout).map_err(|e| {
            error::AnyframeError::SourceError(format!("Invalid UTF-8 in status output: {}", e))
        })?;

        // Process status output
        let processed_output = status_str
            .lines()
            .map(|line| {
                if line.len() >= 3 {
                    let status = &line[0..2];
                    let file_path = &line[3..];
                    format!("{}\t{}{}", status, base_path, file_path)
                } else {
                    line.to_string()
                }
            })
            .filter(|line| {
                if let Some(pattern) = &self.pattern {
                    // If pattern is specified, include only lines that start with the pattern
                    line.starts_with(pattern)
                } else {
                    // If no pattern is specified, include all lines
                    true
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(processed_output)
    }

    fn name(&self) -> &str {
        "git-status"
    }
}

// Similar implementations for other sources to be added
