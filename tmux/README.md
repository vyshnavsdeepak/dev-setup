# Tmux

## Prefix

`Ctrl+a` (remapped from default `Ctrl+b` for easier reach).

## Keybindings

### Panes

| Key | Action |
|---|---|
| `prefix \|` | Split vertically |
| `prefix -` | Split horizontally |
| `prefix h/j/k/l` | Navigate panes (vim-style) |
| `prefix H/J/K/L` | Resize panes (repeatable) |

### Windows

| Key | Action |
|---|---|
| `prefix c` | New window (in current path) |
| `prefix 1-9` | Switch to window |

### Copy mode

| Key | Action |
|---|---|
| `prefix [` | Enter copy mode (vi keys) |
| `v` | Begin selection |
| `y` | Copy to system clipboard (pbcopy) |

### Other

| Key | Action |
|---|---|
| `prefix r` | Reload config |
| `prefix I` | Install plugins (TPM) |

## Plugins

| Plugin | What it does |
|---|---|
| `tmux-sensible` | Universal sane defaults |
| `tmux-resurrect` | Save and restore sessions manually |
| `tmux-continuum` | Auto-saves sessions every 15 min, restores on start |
| `catppuccin/tmux` | Catppuccin Mocha theme |

## Settings

- Mouse enabled
- History limit: 10,000 lines
- Windows and panes start at index 1
- Windows auto-renumber on close
- Vi mode for copy
- Zero escape time (no delay after prefix)

## Files

- `tmux.conf` — main config (`~/.tmux.conf`)
- Plugins managed by TPM at `~/.tmux/plugins/tpm`
