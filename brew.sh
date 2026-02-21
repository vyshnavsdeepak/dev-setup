#!/bin/bash
set -e

echo "==> Installing Homebrew packages"

# Languages & runtimes
brew install fnm go neovim

# Containers
brew install docker docker-compose colima

# CLI tools
brew install fzf bat eza fd ripgrep jq zoxide starship tmux lazygit lazydocker tldr gnupg glow

# Terminal
brew install --cask ghostty

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
