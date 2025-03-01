//! Actions for anyframe-rs
//!
//! Actions perform operations on selected items, such as executing, inserting, or putting them.

use crate::Result;

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

    fn name(&self) -> &'static str {
        "execute"
    }
}

// Similar implementations for other actions to be added
