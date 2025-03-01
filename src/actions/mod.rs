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
///
/// Executes the selected command in zsh.
/// Similar to the original anyframe-action-execute function.
pub struct Execute;

impl Action for Execute {
    fn perform(&self, item: &str) -> Result<()> {
        // Execute the selected command in zsh
        // We use BUFFER and accept-line to execute the command in zsh
        // This is similar to how the original anyframe-action-execute function works
        let execute_output = Command::new("zsh")
            .arg("-c")
            .arg(format!(
                "BUFFER=\"{}\"; zle accept-line 2>/dev/null || eval \"$BUFFER\"",
                item.replace("\"", "\\\"").replace("$", "\\$")
            ))
            .output()
            .map_err(|e| {
                error::AnyframeError::ActionError(format!("Failed to execute command: {}", e))
            })?;

        if !execute_output.status.success() {
            return Err(error::AnyframeError::ActionError(format!(
                "Execute command failed: {}",
                String::from_utf8_lossy(&execute_output.stderr)
            )));
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "execute"
    }
}

/// Insert action
pub struct Insert;

impl Action for Insert {
    fn perform(&self, item: &str) -> Result<()> {
        // Implementation to insert the selected item at the cursor position in zsh
        let insert_output = Command::new("zsh")
            .arg("-c")
            .arg(format!("print -z \"{}\"", item))
            .output()
            .map_err(|e| {
                error::AnyframeError::ActionError(format!(
                    "Failed to execute insert command: {}",
                    e
                ))
            })?;

        if !insert_output.status.success() {
            return Err(error::AnyframeError::ActionError(format!(
                "Insert command failed: {}",
                String::from_utf8_lossy(&insert_output.stderr)
            )));
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "insert"
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

    fn name(&self) -> &'static str {
        "change-directory"
    }
}
// Similar implementations for other actions to be added
