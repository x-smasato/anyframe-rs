//! anyframe-rs: A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh

use anyframe_rs::{
    actions::Execute,
    selectors::Peco,
    sources::History,
    widgets::{ExecuteHistory, Widget},
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a command from history
    ExecuteHistory,
    // Other commands to be added
}

fn main() -> anyframe_rs::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ExecuteHistory => {
            let source = History;
            let selector = Peco::new(None);
            let action = Execute;
            let widget = ExecuteHistory::new(source, selector, action);
            widget.run()?;
        }
    }

    Ok(())
}
