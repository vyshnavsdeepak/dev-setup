use async_trait::async_trait;
use color_eyre::Result;
use tokio::sync::mpsc;

use super::{check_command, run_command, InstallerEvent, InstallStep};

pub struct PostInstallStep {
    #[allow(dead_code)]
    pub dotfiles_dir: String,
}

impl PostInstallStep {
    pub fn new(dotfiles_dir: &str) -> Self {
        Self {
            dotfiles_dir: dotfiles_dir.to_string(),
        }
    }
}

async fn log(tx: &mpsc::Sender<InstallerEvent>, step_id: &str, line: &str) {
    let _ = tx
        .send(InstallerEvent::LogLine {
            step_id: step_id.to_string(),
            line: line.to_string(),
        })
        .await;
}

#[async_trait]
impl InstallStep for PostInstallStep {
    fn id(&self) -> &str {
        "post"
    }

    fn label(&self) -> &str {
        "post-install (omz, tpm, node, colima)"
    }

    fn estimated_secs(&self) -> u64 {
        120
    }

    async fn is_already_done(&self) -> bool {
        let home = dirs::home_dir().unwrap_or_default();
        let omz_done = home.join(".oh-my-zsh").exists();
        let tpm_done = home.join(".tmux/plugins/tpm").exists();
        let colima_done = check_command("colima", &["status"]).await;
        omz_done && tpm_done && colima_done
    }

    async fn execute(&self, tx: mpsc::Sender<InstallerEvent>, dry_run: bool) -> Result<()> {
        let step_id = self.id();
        let home = dirs::home_dir().unwrap_or_default();

        // --- Oh My Zsh ---
        let omz_dir = home.join(".oh-my-zsh");
        if omz_dir.exists() {
            log(&tx, step_id, "[ok] oh-my-zsh already installed").await;
        } else if dry_run {
            log(&tx, step_id, "[dry-run] install oh-my-zsh --unattended").await;
        } else {
            log(&tx, step_id, "installing oh-my-zsh...").await;
            let ok = run_command(
                "sh",
                &[
                    "-c",
                    "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh) '' --unattended",
                ],
                step_id,
                &tx,
            )
            .await?;
            if !ok {
                color_eyre::eyre::bail!("oh-my-zsh install failed");
            }
        }

        // --- TPM ---
        let tpm_dir = home.join(".tmux/plugins/tpm");
        if tpm_dir.exists() {
            log(&tx, step_id, "[ok] tpm already installed").await;
        } else if dry_run {
            log(
                &tx,
                step_id,
                "[dry-run] git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm",
            )
            .await;
        } else {
            log(&tx, step_id, "cloning tpm...").await;
            run_command(
                "git",
                &[
                    "clone",
                    "https://github.com/tmux-plugins/tpm",
                    &tpm_dir.to_string_lossy(),
                ],
                step_id,
                &tx,
            )
            .await?;
        }

        // --- gh-dash extension ---
        if dry_run {
            log(&tx, step_id, "[dry-run] gh extension install dlvhdr/gh-dash").await;
        } else {
            log(&tx, step_id, "installing gh-dash extension...").await;
            // Ignore errors — may already be installed
            let _ = run_command(
                "gh",
                &["extension", "install", "dlvhdr/gh-dash"],
                step_id,
                &tx,
            )
            .await;
        }

        // --- fnm + Node LTS ---
        if dry_run {
            log(&tx, step_id, "[dry-run] fnm install --lts && fnm default lts-latest").await;
        } else {
            log(&tx, step_id, "installing Node LTS via fnm...").await;
            // fnm needs its env set up first; use a shell wrapper
            let _ = run_command(
                "sh",
                &["-c", "eval \"$(fnm env)\" && fnm install --lts && fnm default lts-latest"],
                step_id,
                &tx,
            )
            .await;
        }

        // --- Colima ---
        let colima_running = check_command("colima", &["status"]).await;
        if colima_running {
            log(&tx, step_id, "[ok] colima already running").await;
        } else if dry_run {
            log(
                &tx,
                step_id,
                "[dry-run] colima start --cpu 4 --memory 8 --disk 60",
            )
            .await;
        } else {
            log(&tx, step_id, "starting colima...").await;
            run_command(
                "colima",
                &["start", "--cpu", "4", "--memory", "8", "--disk", "60"],
                step_id,
                &tx,
            )
            .await?;
        }

        Ok(())
    }
}
