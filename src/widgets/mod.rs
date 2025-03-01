//! Widgets for anyframe-rs
//!
//! Widgets combine sources, selectors, and actions to create useful functionalities.

use crate::{actions::Action, selectors::Selector, sources::Source, Result};

/// Trait for widgets
pub trait Widget {
    /// Run the widget
    fn run(&self) -> Result<()>;

    /// Get the name of the widget
    fn name(&self) -> &str;
}

/// Execute history widget
pub struct ExecuteHistory<S: Source, F: Selector, A: Action> {
    source: S,
    selector: F,
    action: A,
}

impl<S: Source, F: Selector, A: Action> ExecuteHistory<S, F, A> {
    /// Create a new ExecuteHistory widget
    pub fn new(source: S, selector: F, action: A) -> Self {
        Self {
            source,
            selector,
            action,
        }
    }
}

impl<S: Source, F: Selector, A: Action> Widget for ExecuteHistory<S, F, A> {
    fn run(&self) -> Result<()> {
        let data = self.source.get_data()?;
        let selected = self.selector.select(&data, None)?;
        self.action.perform(&selected)?;

        Ok(())
    }

    fn name(&self) -> &str {
        "execute-history"
    }
}

/// Checkout git branch widget
pub struct CheckoutGitBranch<S: Source, F: Selector, A: Action> {
    source: S,
    selector: F,
    action: A,
}

impl<S: Source, F: Selector, A: Action> CheckoutGitBranch<S, F, A> {
    /// Create a new CheckoutGitBranch widget
    pub fn new(source: S, selector: F, action: A) -> Self {
        Self {
            source,
            selector,
            action,
        }
    }
}

impl<S: Source, F: Selector, A: Action> Widget for CheckoutGitBranch<S, F, A> {
    fn run(&self) -> Result<()> {
        let data = self.source.get_data()?;
        let selected = self.selector.select(&data, None)?;
        
        // Extract the first field (branch name) from the selected line
        let branch_name = selected.split_whitespace().next().unwrap_or("");
        
        // Execute git checkout command
        self.action.perform(&format!("git checkout {}", branch_name))?;
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "checkout-git-branch"
    }
}

/// Insert git branch widget
pub struct InsertGitBranch<S: Source, F: Selector, A: Action> {
    source: S,
    selector: F,
    action: A,
}

impl<S: Source, F: Selector, A: Action> InsertGitBranch<S, F, A> {
    /// Create a new InsertGitBranch widget
    pub fn new(source: S, selector: F, action: A) -> Self {
        Self {
            source,
            selector,
            action,
        }
    }
}

impl<S: Source, F: Selector, A: Action> Widget for InsertGitBranch<S, F, A> {
    fn run(&self) -> Result<()> {
        let data = self.source.get_data()?;
        let selected = self.selector.select(&data, None)?;
        
        // Extract the first field (branch name) from the selected line
        let branch_name = selected.split_whitespace().next().unwrap_or("");
        
        // Insert branch name
        self.action.perform(branch_name)?;
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "insert-git-branch"
    }
}

/// Git add widget
pub struct GitAdd<S: Source, F: Selector, A: Action> {
    source: S,
    selector: F,
    action: A,
}

impl<S: Source, F: Selector, A: Action> GitAdd<S, F, A> {
    /// Create a new GitAdd widget
    pub fn new(source: S, selector: F, action: A) -> Self {
        Self {
            source,
            selector,
            action,
        }
    }
}

impl<S: Source, F: Selector, A: Action> Widget for GitAdd<S, F, A> {
    fn run(&self) -> Result<()> {
        let data = self.source.get_data()?;
        let selected = self.selector.select(&data, None)?;
        
        // Extract the second field (file path) from the selected line
        let file_path = selected.split_whitespace().nth(1).unwrap_or("");
        
        // Execute git add command
        self.action.perform(&format!("git add --{}", file_path))?;
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "git-add"
    }
}

// Similar implementations for other widgets to be added
