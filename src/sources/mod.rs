//! Sources for anyframe-rs
//!
//! Sources provide data to be filtered, such as command history, directories, processes, etc.

use crate::Result;

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
        // Implementation to get command history
        // This will need to interact with zsh to get the history

        Ok("Command history data".to_string()) // Placeholder
    }

    fn name(&self) -> &str {
        "history"
    }
}

// Similar implementations for other sources to be added
