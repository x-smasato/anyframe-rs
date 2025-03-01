//! anyframe-rs: A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh

pub mod actions;
pub mod selectors;
pub mod sources;
pub mod widgets;

/// Core error types for anyframe-rs
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum AnyframeError {
        #[error("Selector not found: {0}")]
        SelectorNotFound(String),

        #[error("Source error: {0}")]
        SourceError(String),

        #[error("Action error: {0}")]
        ActionError(String),

        #[error("Widget error: {0}")]
        WidgetError(String),

        #[error("IO error: {0}")]
        IoError(#[from] std::io::Error),
    }
}

/// Result type for anyframe-rs operations
pub type Result<T> = std::result::Result<T, error::AnyframeError>;
