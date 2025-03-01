//! anyframe-rs: A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh

use anyframe_rs::{
    actions::{Execute, Insert},
    selectors::Peco,
    sources::{GitBranch, GitStatus, History},
    widgets::{CheckoutGitBranch, ExecuteHistory, GitAdd, InsertGitBranch, Widget},
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
    /// Checkout a git branch
    CheckoutGitBranch {
        #[arg(short, long)]
        include_current: bool,
        #[arg(short, long)]
        remote: bool,
        #[arg(short, long)]
        all: bool,
    },
    /// Insert a git branch
    InsertGitBranch {
        #[arg(short, long)]
        include_current: bool,
        #[arg(short, long)]
        remote: bool,
        #[arg(short, long)]
        all: bool,
    },
    /// Add files to git
    GitAdd {
        #[arg(short, long)]
        pattern: Option<String>,
    },
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
        Commands::CheckoutGitBranch {
            include_current,
            remote,
            all,
        } => {
            let source = GitBranch::new(!include_current, remote, all);
            let selector = Peco::new(None);
            let action = Execute;
            let widget = CheckoutGitBranch::new(source, selector, action);
            widget.run()?;
        }
        Commands::InsertGitBranch {
            include_current,
            remote,
            all,
        } => {
            let source = GitBranch::new(!include_current, remote, all);
            let selector = Peco::new(None);
            let action = Insert;
            let widget = InsertGitBranch::new(source, selector, action);
            widget.run()?;
        }
        Commands::GitAdd { pattern } => {
            let source = GitStatus::new(pattern);
            let selector = Peco::new(None);
            let action = Execute;
            let widget = GitAdd::new(source, selector, action);
            widget.run()?;
        }
    }

    Ok(())
}
