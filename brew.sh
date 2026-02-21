#!/bin/bash
set -e

DOTFILES_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "==> Installing Homebrew packages"
brew bundle --file="$DOTFILES_DIR/Brewfile"

# Oh My Zsh
if [ ! -d "$HOME/.oh-my-zsh" ]; then
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
fi

# TPM (Tmux Plugin Manager)
if [ ! -d "$HOME/.tmux/plugins/tpm" ]; then
  git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm
fi

# Node LTS
eval "$(fnm env)"
fnm install --lts
fnm default lts-latest

# Start Colima (Docker runtime)
colima start --cpu 4 --memory 8 --disk 60

echo "==> Done! Run ./install.sh to link dotfiles"
