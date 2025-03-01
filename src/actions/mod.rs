//! Actions for anyframe-rs
//!
//! Actions perform operations on selected items, such as executing, inserting, or putting them.

use crate::{error, Result};
use std::process::Command;

/// Trait for actions
pub trait Action {
    /// Perform the action on the selected item
    fn perform(&self, item: &str) -> Result<()>;

    /// Get the name of the action
    fn name(&self) -> &str;
}

/// Execute action
pub struct Execute;

impl Action for Execute {
    fn perform(&self, _item: &str) -> Result<()> {
        // Implementation to execute the selected command
        // This will need to interact with zsh to execute the command

        Ok(()) // Placeholder
    }

    fn name(&self) -> &str {
        "execute"
    }
}

/// Change directory action
pub struct ChangeDirectory;

impl Action for ChangeDirectory {
    fn perform(&self, item: &str) -> Result<()> {
        // Change directory to the selected repository
        // This will need to interact with zsh to change the directory
        let cd_output = Command::new("zsh")
            .arg("-c")
            .arg(format!("cd {}", item))
            .output()
            .map_err(|e| {
                error::AnyframeError::ActionError(format!("Failed to execute cd command: {}", e))
            })?;

        if !cd_output.status.success() {
            return Err(error::AnyframeError::ActionError(format!(
                "cd command failed: {}",
                String::from_utf8_lossy(&cd_output.stderr)
            )));
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "change-directory"
    }
}

// Similar implementations for other actions to be added
