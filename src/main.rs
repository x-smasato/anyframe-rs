//! anyframe-rs: A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh

use anyframe_rs::{
    actions::{ChangeDirectory, Execute},
    selectors::Peco,
    sources::{GhqRepository, History},
    widgets::{CdGhqRepository, ExecuteHistory, Widget},
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
    /// Change directory to a ghq repository
    CdGhqRepository,
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
        Commands::CdGhqRepository => {
            let source = GhqRepository;
            let selector = Peco::new(None);
            let action = ChangeDirectory;
            let widget = CdGhqRepository::new(source, selector, action);
            widget.run()?;
        }
    }

    Ok(())
}
