//! anyframe-rs: A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh

use anyframe_rs::{
    actions::Execute,
    selectors::Peco,
    sources::History,
    widgets::{
        ExecuteHistory, Widget,
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
