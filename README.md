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
| **Git** | `git/` | Dual identity (work/personal) via includeIf, GPG signing |
| **Shell** | `shell/` | Oh My Zsh + Starship, fnm, zoxide, CLI aliases |
| **Starship** | `shell/starship.toml` | user@host path -- branch %, industrial minimal |
| **Tmux** | `tmux/` | Ctrl+a prefix, vim nav, resurrect, custom status bar |
| **Neovim** | `nvim/` | LazyVim with LSP, Telescope, Treesitter |
| **Ghostty** | `ghostty/` | SF Mono, muted palette, no theme |

## Setup on a new machine

```bash
# 1. Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Clone this repo
mkdir -p ~/src/github.com/vyshnavsdeepak
git clone git@github.com:vyshnavsdeepak/dev-setup.git ~/src/github.com/vyshnavsdeepak/dev-setup
cd ~/src/github.com/vyshnavsdeepak/dev-setup

# 3. Install packages + start Colima (Docker runtime)
./brew.sh

# 4. Symlink dotfiles
./install.sh

# 5. Set hostname
sudo scutil --set HostName dock
sudo scutil --set LocalHostName dock
sudo scutil --set ComputerName dock

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
vyshnav@dock .../your-work-org/your-work-project -- main modified
%
```
