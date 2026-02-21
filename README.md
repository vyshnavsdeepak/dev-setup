# dev-setup

My macOS development environment dotfiles and setup scripts.

## What's included

| Component | Config | Details |
|---|---|---|
| **Git** | `git/` | Dual identity (work/personal) via `includeIf`, GPG signing |
| **Zsh** | `shell/` | Oh My Zsh + Starship prompt, fnm, zoxide, CLI aliases |
| **Tmux** | `tmux/` | Ctrl+a prefix, vim navigation, resurrect + continuum |
| **Neovim** | `nvim/` | LazyVim with LSP, Telescope, Treesitter |
| **Ghostty** | `ghostty/` | JetBrains Mono, catppuccin mocha, tmux-friendly |

## Setup on a new machine

```bash
# 1. Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Clone this repo
mkdir -p ~/src/github.com/vyshnavsdeepak
git clone git@github.com:vyshnavsdeepak/dev-setup.git ~/src/github.com/vyshnavsdeepak/dev-setup
cd ~/src/github.com/vyshnavsdeepak/dev-setup

# 3. Install packages
./brew.sh

# 4. Symlink dotfiles
./install.sh

# 5. Install tmux plugins (inside tmux)
# prefix + I

# 6. Open nvim (plugins auto-install)
nvim
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

| Directory | Email |
|---|---|
| `~/src/github.com/your-work-org/` | user@work.example.com |
| `~/src/github.com/vyshnavsdeepak/` | user@personal.example.com |
