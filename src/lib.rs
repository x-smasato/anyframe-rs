//! anyframe-rs: A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::unnecessary_literal_unwrap)]
#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::must_use_candidate)]

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
