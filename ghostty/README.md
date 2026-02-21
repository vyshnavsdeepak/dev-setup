# Ghostty

## Why Ghostty

GPU-accelerated (native Metal), fastest terminal on macOS. No built-in multiplexer — pairs perfectly with tmux. Created by Mitchell Hashimoto (HashiCorp founder) who uses tmux + neovim himself.

## Settings

| Setting | Value | Why |
|---|---|---|
| `font-family` | JetBrains Mono | Excellent monospace font with ligatures |
| `font-size` | 14 | Comfortable for long coding sessions |
| `theme` | catppuccin-mocha | Matches tmux and neovim theme |
| `macos-titlebar-style` | tabs | Native macOS tab bar |
| `macos-option-as-alt` | true | Needed for terminal keybindings |
| `cursor-style` | block, no blink | Matches neovim block cursor |
| `copy-on-select` | clipboard | Select text to copy automatically |

## Unbound keys

Ghostty's own split/tab keybindings are disabled since tmux handles multiplexing:

- `Cmd+D` (vertical split) — unbound
- `Cmd+Shift+D` (horizontal split) — unbound
- `Cmd+T` (new tab) — unbound

## Files

- `config` — Ghostty config (`~/.config/ghostty/config`)
