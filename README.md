# dev-setup

macOS development environment. Industrial minimal.

## Design

- One accent color (#cc7832)
- No icons, no emojis, no gradients
- SF Mono, muted ANSI palette
- Everything serves a function

## What's included

| Component | Config | Details |
|---|---|---|
| **Git** | `git/` | Dual identity (work/personal) via includeIf, GPG signing, delta diffs |
| **Shell** | `shell/` | Oh My Zsh + Starship, fnm, zoxide, CLI aliases |
| **Starship** | `shell/starship.toml` | user@host path -- branch %, industrial minimal |
| **Tmux** | `tmux/` | Ctrl+a prefix, vim nav, resurrect, custom status bar |
| **Neovim** | `nvim/` | LazyVim with LSP, Telescope, Treesitter |
| **Ghostty** | `ghostty/` | SF Mono, muted palette, no theme |

## Tools

### Modern unix replacements

| Tool | Replaces | What you get |
|---|---|---|
| `bat` | `cat` | Syntax highlighting, line numbers, git integration |
| `eza` | `ls` | Colors, git status, tree view |
| `fd` | `find` | Simpler syntax, respects .gitignore |
| `ripgrep` | `grep` | Fast, recursive, respects .gitignore |
| `fzf` | - | Fuzzy finder for files, history, anything |
| `zoxide` | `cd` | Smart directory jumping, learns from usage |
| `dust` | `du` | Visual disk usage with bar charts |
| `procs` | `ps` | Colored, searchable, tree view process list |
| `gping` | `ping` | Live graph of ping latency, overlay multiple hosts |
| `jq` | - | JSON processor for the command line |
| `grex` | - | Generate regex from example strings |
| `tldr` | `man` | Simplified, example-driven man pages |

### TUI dashboards

| Tool | What you see |
|---|---|
| `btop` | Live CPU, RAM, disk, network, process dashboard |
| `bandwhich` | Per-process network bandwidth usage (`sudo bandwhich`) |
| `trippy` | Live traceroute TUI — watch packets hop across the network (`sudo trip`) |
| `lazygit` | Full git TUI — staging, commits, branches, stash |
| `gitui` | Lightweight alternative git TUI |
| `lazydocker` | Docker containers, images, logs, stats |
| `wtfutil` | Configurable personal dashboard — GitHub, system stats, custom widgets |
| `gh dash` | GitHub dashboard TUI — PRs, issues, CI status across repos |
| `process-compose` | Multi-process orchestrator TUI with live logs |

### Dev tools

| Tool | What it does |
|---|---|
| `neovim` | Editor — LazyVim distribution with IDE features |
| `tmux` | Terminal multiplexer — splits, sessions, persistence |
| `go` | Go toolchain |
| `fnm` | Fast Node.js version manager (auto-switches on cd) |
| `docker` + `colima` | Container runtime (no Docker Desktop needed) |
| `docker-compose` | Multi-container orchestration |
| `gh` | GitHub CLI |
| `act` | Run GitHub Actions workflows locally |
| `git-delta` | Beautiful git diffs — side-by-side, syntax highlighting |
| `gnupg` | GPG for commit signing |
| `just` | Command runner (simpler Make) |
| `overmind` | Procfile-based process manager |
| `glow` | Markdown renderer for the terminal |
| `starship` | Cross-shell prompt |

### Apps (casks)

| App | What it does |
|---|---|
| **Ghostty** | Terminal emulator |
| **Warp** | AI-powered terminal |
| **Raycast** | Launcher, window management, clipboard, snippets |
| **Obsidian** | Markdown-based knowledge base |
| **Superwhisper** | Voice-to-text, runs locally |
| **Shottr** | Screenshots with annotation, measurement, blur |
| **Figma** | Design tool |
| **Slack** | Messaging |
| **VS Code** | Editor (fallback) |
| **Deskflow** | KVM — share keyboard/mouse across machines |

## Setup on a new machine

```bash
# 1. Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Clone this repo
git clone git@github.com:vyshnavsdeepak/dev-setup.git ~/dev-setup
cd ~/dev-setup

# 3. Install packages + start Colima (Docker runtime)
./brew.sh

# 4. Symlink dotfiles
./install.sh

# 5. Set hostname (optional)
sudo scutil --set HostName yourhost
sudo scutil --set LocalHostName yourhost
sudo scutil --set ComputerName yourhost

# 6. Install tmux plugins (inside tmux)
# Ctrl+a then Shift+I

# 7. Open nvim (plugins auto-install)
# nvim, then :LazyExtras to enable lang.go, lang.typescript, lang.docker
```

## Docker

Colima (free, open source) as the Docker runtime. No Docker Desktop needed.

```bash
colima start --cpu 4 --memory 8 --disk 60
```

## CLI aliases

| Alias | Tool |
|---|---|
| `cat` | `bat` |
| `ls` / `ll` / `lt` | `eza` |
| `find` | `fd` |
| `grep` | `rg` |
| `lg` | `lazygit` |
| `ld` | `lazydocker` |
| `z` | `zoxide` |

## Git identity

Automatically switches based on project directory:

| Directory | Email | GPG Key |
|---|---|---|
| `~/src/github.com/your-work-org/` | user@work.example.com | `YOUR_WORK_GPG_KEY_ID` |
| `~/src/github.com/vyshnavsdeepak/` | user@personal.example.com | `YOUR_PERSONAL_GPG_KEY_ID` |

## Prompt

```
user@host .../your-work-org/your-work-project -- main modified
%
```

## Troubleshooting

### Colima fails to start — "disk in use by instance colima"

Happens after a crash or unclean shutdown. Lima leaves a stale symlink:

```bash
rm ~/.colima/_lima/_disks/colima/in_use_by
brew services restart colima
```

### GPG signing fails — "database_open waiting for lock"

Stale lock file left after a crash:

```bash
rm ~/.gnupg/public-keys.d/pubring.db.lock
```
