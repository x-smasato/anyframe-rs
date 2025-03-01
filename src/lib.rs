//! # anyframe-rs
//!
//! A Rust implementation of [anyframe](https://github.com/x-smasato/anyframe), a peco/percol/fzf wrapper plugin for zsh.
//!
//! ## Overview
//!
//! anyframe-rs provides interactive filtering of various data sources (command history, directories, processes, etc.)
//! using popular filtering tools like peco, percol, fzf, and fzf-tmux. It is designed to be used as a Zsh plugin,
//! providing widgets for common operations like executing commands from history, changing directories, and more.
//!
//! ## Architecture
//!
//! anyframe-rs consists of four main components:
//!
//! - **Sources**: Provide data to be filtered (history, directories, processes, etc.)
//! - **Selectors**: Interactive filtering tools (peco, percol, fzf, fzf-tmux)
//! - **Actions**: Perform operations on selected items (execute, insert, put)
//! - **Widgets**: Combine sources, selectors, and actions to create useful functionalities
//!
//! ## Example
//!
//! ```rust,no_run
//! use anyframe_rs::{
//!     actions::Execute,
//!     selectors::Fzf,
//!     sources::History,
//!     widgets::{ExecuteHistory, Widget},
//! };
//!
//! fn main() -> anyframe_rs::Result<()> {
//!     let source = History;
//!     let selector = Fzf::new(None);
//!     let action = Execute;
//!     let widget = ExecuteHistory::new(source, selector, action);
//!     widget.run()?;
//!     Ok(())
//! }
//! ```

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

    /// Errors that can occur in anyframe-rs operations
    #[derive(Error, Debug)]
    pub enum AnyframeError {
        /// Error when a selector is not found
        #[error("Selector not found: {0}")]
        SelectorNotFound(String),

        /// Error from a source
        #[error("Source error: {0}")]
        SourceError(String),

        /// Error from an action
        #[error("Action error: {0}")]
        ActionError(String),

        /// Error from a widget
        #[error("Widget error: {0}")]
        WidgetError(String),

        /// IO error
        #[error("IO error: {0}")]
        IoError(#[from] std::io::Error),
    }
}

/// Result type for anyframe-rs operations
pub type Result<T> = std::result::Result<T, error::AnyframeError>;
