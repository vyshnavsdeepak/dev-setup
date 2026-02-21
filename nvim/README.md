# Neovim (LazyVim)

## Distribution

[LazyVim](https://www.lazyvim.org/) — a pre-configured Neovim setup with sensible defaults and a plugin manager (lazy.nvim).

## Key mappings

Leader key is `Space`.

### Navigation

| Key | Action |
|---|---|
| `Space f f` | Find files (Telescope) |
| `Space f g` | Live grep across files |
| `Space f r` | Recent files |
| `Space e` | File explorer (neo-tree) |
| `Space ,` | Switch buffers |

### Code

| Key | Action |
|---|---|
| `Space c a` | Code actions |
| `Space c r` | Rename symbol |
| `g d` | Go to definition |
| `g r` | Go to references |
| `K` | Hover documentation |
| `Space l` | LSP menu |

### Git

| Key | Action |
|---|---|
| `Space g g` | LazyGit (inside nvim) |
| `Space g b` | Git blame line |

### Other

| Key | Action |
|---|---|
| `Space` | Which-key menu (shows all keybindings) |
| `:LazyExtras` | Browse and enable extra plugin packs |
| `:Lazy` | Plugin manager UI |

## Recommended LazyExtras

Enable these for your stack via `:LazyExtras`:

- `lang.go` — Go LSP, formatting, test runner
- `lang.typescript` — TS/JS LSP, formatting
- `lang.docker` — Dockerfile support

## Structure

```
nvim/
├── init.lua              # Entry point, loads lazy.nvim
├── lua/
│   ├── config/
│   │   ├── autocmds.lua  # Custom autocommands
│   │   ├── keymaps.lua   # Custom keymaps
│   │   ├── lazy.lua      # Lazy.nvim bootstrap
│   │   └── options.lua   # Vim options
│   └── plugins/
│       └── example.lua   # Add custom plugin specs here
├── lazy-lock.json        # Plugin version lock file
└── lazyvim.json          # LazyExtras config
```

## Adding custom plugins

Create a new file in `lua/plugins/`, e.g. `lua/plugins/my-plugin.lua`:

```lua
return {
  { "author/plugin-name", opts = {} },
}
```

## Files

- Config dir: `~/.config/nvim/` (symlinked from this repo)
