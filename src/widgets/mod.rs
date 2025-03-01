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

    fn name(&self) -> &'static str {
        "execute-history"
    }
}

// Similar implementations for other widgets to be added
