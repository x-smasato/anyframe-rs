//! anyframe-rs: A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh

use anyframe_rs::{
    actions::{ChangeDirectory, Execute, Insert},
    selectors::{Fzf, FzfTmux, Peco, Percol},
    sources::{Cdr, GhqRepository, GitBranch, GitStatus, History, Process},
    widgets::{
        CdGhqRepository, CheckoutGitBranch, ExecuteHistory, GitAdd, InsertGitBranch, Widget,
    },
};

fn main() -> anyframe_rs::Result<()> {
    let source = History;
    let selector = Peco::new(None);
    let action = Execute;
    let widget = ExecuteHistory::new(source, selector, action);
    widget.run()?;
    Ok(())
}
