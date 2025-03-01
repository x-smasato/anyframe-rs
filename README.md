<div align="center">
<h1>anyframe-rs</h1>
<p><em>A Rust implementation of anyframe, a peco/percol/fzf wrapper plugin for zsh</em></p>
</div>

## What is it?

anyframe-rs is a Rust implementation of [anyframe](https://github.com/x-smasato/anyframe), providing:

- Interactive filtering of various data sources (command history, directories, processes, etc.)
- Integration with popular filtering tools like [peco](https://github.com/peco/peco), [percol](https://github.com/mooz/percol), [fzf](https://github.com/junegunn/fzf), and fzf-tmux
- Zsh widgets for common operations like executing commands from history, changing directories, and more
- Better performance and reliability through Rust implementation

## Installation

### Prerequisites

First, you need to install one of the following filtering tools:
- [peco](https://github.com/peco/peco)
- [percol](https://github.com/mooz/percol)
- [fzf](https://github.com/junegunn/fzf)
- fzf-tmux

### From Source

```sh-session
$ git clone https://github.com/x-smasato/anyframe-rs
$ cd anyframe-rs
$ cargo build --release
$ cp target/release/anyframe-rs ~/.local/bin/
```

## Usage

### Basic Setup

Add the following to your `.zshrc`:

```zsh
# Load anyframe-rs plugin
source /path/to/anyframe-rs/anyframe.plugin.zsh
```

### Keybindings

You can map anyframe-rs widgets to whatever key you like:

```zsh
bindkey '^xr' anyframe-widget-execute-history
bindkey '^xd' anyframe-widget-cd-directory
# Add more keybindings as needed
```

## Components

anyframe-rs consists of four main components:

- **Sources**: Provide data to be filtered (history, directories, processes, etc.)
- **Selectors**: Interactive filtering tools (peco, percol, fzf, fzf-tmux)
- **Actions**: Perform operations on selected items (execute, insert, put)
- **Widgets**: Combine sources, selectors, and actions to create useful functionalities

## Configuration

```zsh
# Specify which selector to use
zstyle ":anyframe:selector:" use peco
# or
zstyle ":anyframe:selector:" use fzf

# Specify path and options for selectors
zstyle ":anyframe:selector:peco:" command 'peco --no-ignore-case'
zstyle ":anyframe:selector:fzf:" command 'fzf --extended'
```

## Examples

### Execute a command from history

```zsh
anyframe-widget-execute-history
```

### Change to a directory

```zsh
anyframe-widget-cd-directory
```

### Checkout a git branch

```zsh
anyframe-widget-checkout-git-branch
```

### Insert a git branch name

```zsh
anyframe-widget-insert-git-branch
```

### Add files to git

```zsh
anyframe-widget-git-add
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to anyframe-rs.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Development

### Pre-commit hooks

This project uses [pre-commit](https://pre-commit.com/) to run automatic checks before each commit.

To install pre-commit:

```bash
pip install pre-commit
pre-commit install
```

To run pre-commit manually:

```bash
pre-commit run --all-files
```
