//! Sources for anyframe-rs
//! 
//! Sources provide data to be filtered, such as command history, directories, processes, etc.

use crate::{Result, error};
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
            .map_err(|e| error::AnyframeError::SourceError(format!("Failed to execute zsh: {}", e)))?;
            
        if !history_output.status.success() {
            return Err(error::AnyframeError::SourceError(
                format!("zsh command failed: {}", String::from_utf8_lossy(&history_output.stderr))
            ));
        }
        
        let history_str = String::from_utf8(history_output.stdout)
            .map_err(|e| error::AnyframeError::SourceError(format!("Invalid UTF-8 in history output: {}", e)))?;
            
        Ok(history_str)
    }
    
    fn name(&self) -> &str {
        "history"
    }
}

// Similar implementations for other sources to be added
