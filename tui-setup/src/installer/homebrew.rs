use async_trait::async_trait;
use color_eyre::Result;
use tokio::sync::mpsc;

use super::{check_command, run_command, InstallerEvent, InstallStep};

pub struct HomebrewStep {
    pub dotfiles_dir: String,
}

impl HomebrewStep {
    pub fn new(dotfiles_dir: &str) -> Self {
        Self {
            dotfiles_dir: dotfiles_dir.to_string(),
        }
    }

    fn brewfile_path(&self) -> String {
        format!("{}/Brewfile", self.dotfiles_dir)
    }
}

#[async_trait]
impl InstallStep for HomebrewStep {
    fn id(&self) -> &str {
        "homebrew"
    }

    fn label(&self) -> &str {
        "homebrew packages"
    }

    fn estimated_secs(&self) -> u64 {
        180
    }

    async fn is_already_done(&self) -> bool {
        check_command("brew", &["bundle", "check", "--file", &self.brewfile_path()]).await
    }

    async fn execute(&self, tx: mpsc::Sender<InstallerEvent>, dry_run: bool) -> Result<()> {
        let step_id = self.id();
        let brewfile = self.brewfile_path();

        if dry_run {
            let _ = tx
                .send(InstallerEvent::LogLine {
                    step_id: step_id.to_string(),
                    line: format!("[dry-run] brew bundle install --file={brewfile}"),
                })
                .await;
            return Ok(());
        }

        let ok = run_command(
            "brew",
            &["bundle", "install", "--file", &brewfile],
            step_id,
            &tx,
        )
        .await?;

        if !ok {
            color_eyre::eyre::bail!("brew bundle install failed");
        }

        Ok(())
    }
}
