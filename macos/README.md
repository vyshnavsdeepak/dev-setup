# macOS Dev Defaults

System-level settings optimized for software development.

## What changes

### Keyboard

| Setting | Default | Dev | Why |
|---|---|---|---|
| KeyRepeat | 6 (slow) | 2 (fast) | Hold j/k in vim without waiting |
| InitialKeyRepeat | 25 (slow) | 15 (fast) | Less delay before repeat starts |
| PressAndHold | on | off | Key repeat works everywhere, no accent popup |

### Finder

| Setting | Default | Dev | Why |
|---|---|---|---|
| ShowAllFiles | off | on | See .env, .git, dotfiles |
| PathBar | off | on | Path breadcrumb at bottom |
| StatusBar | off | on | File count and disk space |
| ShowExtensions | off | on | No guessing file types |

### Text Input

| Setting | Default | Dev | Why |
|---|---|---|---|
| AutoCorrect | on | off | Stops mangling code and commands |
| SmartQuotes | on | off | `"` stays `"`, not curly quotes |
| SmartDashes | on | off | `--` stays `--`, not em-dash |

## Apply

```bash
./macos.sh
```

<details>
<summary>View macos.sh</summary>

```bash
#!/bin/bash
set -e

echo "==> Setting macOS dev defaults"

# Faster key repeat (needs logout to take effect)
defaults write NSGlobalDomain KeyRepeat -int 2
defaults write NSGlobalDomain InitialKeyRepeat -int 15

# Disable press-and-hold for keys (enables key repeat in all apps)
defaults write NSGlobalDomain ApplePressAndHoldEnabled -bool false

# Show hidden files in Finder
defaults write com.apple.finder AppleShowAllFiles -bool true

# Show path bar and status bar in Finder
defaults write com.apple.finder ShowPathbar -bool true
defaults write com.apple.finder ShowStatusBar -bool true

# Show file extensions
defaults write NSGlobalDomain AppleShowAllExtensions -bool true

# Disable auto-correct
defaults write NSGlobalDomain NSAutomaticSpellingCorrectionEnabled -bool false

# Disable smart quotes and dashes (breaks code)
defaults write NSGlobalDomain NSAutomaticQuoteSubstitutionEnabled -bool false
defaults write NSGlobalDomain NSAutomaticDashSubstitutionEnabled -bool false

# Restart Finder
killall Finder

echo "==> Done! Logout and back in for key repeat changes."
```

</details>

## Revert

```bash
./revert.sh
```

<details>
<summary>View revert.sh</summary>

```bash
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
```

</details>

## Notes

- Key repeat changes require **logout and back in**
- All changes persist across reboots
- To check any current value: `defaults read <domain> <key>`
