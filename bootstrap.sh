#!/bin/bash
# bootstrap.sh — Install Rust if needed, then build and run tui-setup
set -e

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
SETUP_DIR="$REPO_DIR/tui-setup"
BINARY="$SETUP_DIR/target/release/setup"

echo "==> dev-setup bootstrap"

# 1. Install rustup + toolchain if missing
if ! command -v rustup &>/dev/null; then
  echo "==> installing rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
  # Source cargo env for this session
  source "$HOME/.cargo/env"
elif ! command -v cargo &>/dev/null; then
  # rustup present but cargo not in PATH
  source "$HOME/.cargo/env"
fi

# 2. Build tui-setup
echo "==> building setup binary..."
cargo build --release --manifest-path "$SETUP_DIR/Cargo.toml"

# 3. Run
echo "==> launching setup..."
exec "$BINARY" "$@"
