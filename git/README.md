# Git

## Dual identity

Uses `includeIf` to auto-switch email and GPG key based on project directory.

| Directory | Email | GPG Key |
|---|---|---|
| `~/src/github.com/your-work-org/` | user@work.example.com | `YOUR_WORK_GPG_KEY_ID` |
| `~/src/github.com/vyshnavsdeepak/` | user@personal.example.com | `YOUR_PERSONAL_GPG_KEY_ID` |

Global default is the work email (work). Personal overrides are loaded from `gitconfig-personal` when inside the personal directory.

## SSH routing

Two SSH keys route to the same GitHub host via `~/.ssh/config`:

- `github.com` -> `~/.ssh/id_ed25519_work` (work, default)
- `github-personal` -> `~/.ssh/id_ed25519` (personal)

The personal gitconfig has an `insteadOf` rule that rewrites `git@github.com:` to `git@github-personal:` automatically inside personal repos.

## Commit signing

GPG commit signing is enabled globally. Both identities have their own GPG key uploaded to GitHub.

## Files

- `gitconfig` — global git config (`~/.gitconfig`)
- `gitconfig-personal` — personal overrides (`~/.gitconfig-personal`)
- `gitignore_global` — global ignore patterns (`~/.gitignore_global`)

## Global gitignore

Ignores: `.DS_Store`, editor files (`.idea/`, `.vscode/`, `*.swp`), `.env`, `node_modules/`, `__pycache__/`

## Settings

| Setting | Value | Why |
|---|---|---|
| `pull.rebase` | `true` | Clean linear history |
| `fetch.prune` | `true` | Auto-remove stale remote branches |
| `rerere.enabled` | `true` | Remember conflict resolutions |
| `init.defaultBranch` | `main` | Modern default |
| `core.editor` | `nvim` | Neovim for commits/rebases |
