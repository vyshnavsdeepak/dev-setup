#!/bin/bash
set -e

DOTFILES_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "==> Linking dotfiles from $DOTFILES_DIR"

# Git
ln -sf "$DOTFILES_DIR/git/gitconfig" ~/.gitconfig
ln -sf "$DOTFILES_DIR/git/gitconfig-personal" ~/.gitconfig-personal
ln -sf "$DOTFILES_DIR/git/gitignore_global" ~/.gitignore_global

# Shell
ln -sf "$DOTFILES_DIR/shell/zshrc" ~/.zshrc
mkdir -p ~/.config
ln -sf "$DOTFILES_DIR/shell/starship.toml" ~/.config/starship.toml

# Tmux
ln -sf "$DOTFILES_DIR/tmux/tmux.conf" ~/.tmux.conf

# Ghostty
mkdir -p ~/.config/ghostty
ln -sf "$DOTFILES_DIR/ghostty/config" ~/.config/ghostty/config

# Neovim
rm -rf ~/.config/nvim
ln -sf "$DOTFILES_DIR/nvim" ~/.config/nvim

echo "==> Dotfiles linked!"
echo ""
echo "Next steps:"
echo "  1. Install Homebrew packages:  ./brew.sh"
echo "  2. Install tmux plugins:       prefix + I (inside tmux)"
echo "  3. Open nvim to install plugins automatically"
