#!/bin/bash
set -e

echo "==> Reverting macOS dev defaults to system defaults"

# Keyboard
defaults write NSGlobalDomain KeyRepeat -int 6
defaults write NSGlobalDomain InitialKeyRepeat -int 25
defaults write NSGlobalDomain ApplePressAndHoldEnabled -bool true

# Finder
defaults write com.apple.finder AppleShowAllFiles -bool false
defaults write com.apple.finder ShowPathbar -bool false
defaults write com.apple.finder ShowStatusBar -bool false
defaults write NSGlobalDomain AppleShowAllExtensions -bool false

# Text input
defaults write NSGlobalDomain NSAutomaticSpellingCorrectionEnabled -bool true
defaults write NSGlobalDomain NSAutomaticQuoteSubstitutionEnabled -bool true
defaults write NSGlobalDomain NSAutomaticDashSubstitutionEnabled -bool true

killall Finder

echo "==> Reverted. Logout and back in for key repeat changes."
