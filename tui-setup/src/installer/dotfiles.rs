use async_trait::async_trait;
use color_eyre::Result;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

use super::{InstallerEvent, InstallStep};

pub struct DotfilesStep {
    pub dotfiles_dir: String,
}

impl DotfilesStep {
    pub fn new(dotfiles_dir: &str) -> Self {
        Self {
            dotfiles_dir: dotfiles_dir.to_string(),
        }
    }

    fn symlink_map(&self) -> Vec<(PathBuf, PathBuf)> {
        let d = &self.dotfiles_dir;
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));

        vec![
            (
                PathBuf::from(format!("{d}/git/gitconfig")),
                home.join(".gitconfig"),
            ),
            (
                PathBuf::from(format!("{d}/git/gitconfig-personal")),
                home.join(".gitconfig-personal"),
            ),
            (
                PathBuf::from(format!("{d}/git/gitignore_global")),
                home.join(".gitignore_global"),
            ),
            (
                PathBuf::from(format!("{d}/shell/zshrc")),
                home.join(".zshrc"),
            ),
            (
                PathBuf::from(format!("{d}/shell/starship.toml")),
                home.join(".config/starship.toml"),
            ),
            (
                PathBuf::from(format!("{d}/tmux/tmux.conf")),
                home.join(".tmux.conf"),
            ),
            (
                PathBuf::from(format!("{d}/ghostty/config")),
                home.join(".config/ghostty/config"),
            ),
            (
                PathBuf::from(format!("{d}/nvim")),
                home.join(".config/nvim"),
            ),
        ]
    }

    fn is_correct_symlink(src: &Path, dst: &Path) -> bool {
        match std::fs::read_link(dst) {
            Ok(target) => target == src,
            Err(_) => false,
        }
    }
}

#[async_trait]
impl InstallStep for DotfilesStep {
    fn id(&self) -> &str {
        "dotfiles"
    }

    fn label(&self) -> &str {
        "dotfiles symlinks"
    }

    fn estimated_secs(&self) -> u64 {
        5
    }

    async fn is_already_done(&self) -> bool {
        self.symlink_map()
            .iter()
            .all(|(src, dst)| Self::is_correct_symlink(src, dst))
    }

    async fn execute(&self, tx: mpsc::Sender<InstallerEvent>, dry_run: bool) -> Result<()> {
        let step_id = self.id();

        for (src, dst) in self.symlink_map() {
            if Self::is_correct_symlink(&src, &dst) {
                let _ = tx
                    .send(InstallerEvent::LogLine {
                        step_id: step_id.to_string(),
                        line: format!("[ok] {} already linked", dst.display()),
                    })
                    .await;
                continue;
            }

            let display = format!("{} -> {}", dst.display(), src.display());

            if dry_run {
                let _ = tx
                    .send(InstallerEvent::LogLine {
                        step_id: step_id.to_string(),
                        line: format!("[dry-run] ln -sf {display}"),
                    })
                    .await;
                continue;
            }

            // Create parent directories
            if let Some(parent) = dst.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Special handling for nvim: only remove if not already a correct symlink
            if dst.ends_with(".config/nvim") {
                if dst.exists() && !dst.is_symlink() {
                    let _ = tx
                        .send(InstallerEvent::LogLine {
                            step_id: step_id.to_string(),
                            line: format!("[warn] removing existing dir {}", dst.display()),
                        })
                        .await;
                    std::fs::remove_dir_all(&dst)?;
                } else if dst.is_symlink() {
                    std::fs::remove_file(&dst)?;
                }
            } else if dst.exists() || dst.is_symlink() {
                std::fs::remove_file(&dst)?;
            }

            std::os::unix::fs::symlink(&src, &dst)?;

            let _ = tx
                .send(InstallerEvent::LogLine {
                    step_id: step_id.to_string(),
                    line: format!("[link] {display}"),
                })
                .await;
        }

        Ok(())
    }
}
