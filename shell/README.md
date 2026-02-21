# Shell (Zsh)

## Framework

Oh My Zsh with Starship prompt (ZSH_THEME disabled).

## Plugins

| Plugin | What it does |
|---|---|
| `git` | Git aliases and completions |
| `golang` | Go aliases and completions |
| `docker` | Docker completions |
| `docker-compose` | Docker Compose completions |
| `fzf` | Fuzzy finder integration |
| `tmux` | Tmux completions and auto-start |

## Tools initialized in .zshrc

| Tool | Init command | What it does |
|---|---|---|
| fnm | `fnm env --use-on-cd` | Node version manager, auto-switches on `cd` |
| zoxide | `zoxide init zsh` | Smarter `cd` — jump with `z dirname` |
| Starship | `starship init zsh` | Cross-shell prompt |

## Aliases

| Alias | Actual command | Tool |
|---|---|---|
| `cat` | `bat` | Syntax-highlighted file viewer |
| `ls` | `eza` | Modern ls with icons |
| `ll` | `eza -la --git` | Detailed listing with git status |
| `lt` | `eza -la --tree --level=2` | Tree view |
| `find` | `fd` | Fast file finder |
| `grep` | `rg` | Ripgrep — fast content search |
| `lg` | `lazygit` | Terminal UI for git |
| `ld` | `lazydocker` | Terminal UI for docker |

## Environment variables

| Variable | Value |
|---|---|
| `EDITOR` | `nvim` |
| `GOPATH` | `$HOME/go` |
| `PATH` | Includes `$HOME/.local/bin`, `$GOPATH/bin` |

## Files

- `zshrc` — main shell config (`~/.zshrc`)
