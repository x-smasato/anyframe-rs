# anyframe.plugin.zsh
# Zsh integration for anyframe-rs

# Path to the anyframe-rs binary
ANYFRAME_RS_PATH="${0:h}/target/release/anyframe-rs"

# Check if the binary exists
if [[ ! -x "$ANYFRAME_RS_PATH" ]]; then
    echo "anyframe-rs binary not found at $ANYFRAME_RS_PATH" >&2
    echo "Please build the project with 'cargo build --release'" >&2
    return 1
fi

# Define widget functions
anyframe-widget-execute-history() {
    "$ANYFRAME_RS_PATH" execute-history
}

# Register widgets with zle
zle -N anyframe-widget-execute-history

# Example keybindings (commented out by default)
# bindkey '^r' anyframe-widget-execute-history
