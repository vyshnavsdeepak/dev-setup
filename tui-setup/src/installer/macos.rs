use async_trait::async_trait;
use color_eyre::Result;
use tokio::sync::mpsc;

use super::{run_command, InstallerEvent, InstallStep};

pub struct MacosStep;

async fn defaults_check(domain: &str, key: &str, expected: &str) -> bool {
    let output = tokio::process::Command::new("defaults")
        .args(["read", domain, key])
        .output()
        .await;

    match output {
        Ok(out) => {
            let val = String::from_utf8_lossy(&out.stdout);
            val.trim() == expected
        }
        Err(_) => false,
    }
}

#[async_trait]
impl InstallStep for MacosStep {
    fn id(&self) -> &str {
        "macos"
    }

    fn label(&self) -> &str {
        "macos defaults"
    }

    fn estimated_secs(&self) -> u64 {
        10
    }

    async fn is_already_done(&self) -> bool {
        // Check a representative key to determine if defaults were applied
        defaults_check("NSGlobalDomain", "KeyRepeat", "2").await
    }

    async fn execute(&self, tx: mpsc::Sender<InstallerEvent>, dry_run: bool) -> Result<()> {
        let step_id = self.id();

        let commands: &[(&str, &[&str])] = &[
            ("defaults", &["write", "NSGlobalDomain", "KeyRepeat", "-int", "2"]),
            ("defaults", &["write", "NSGlobalDomain", "InitialKeyRepeat", "-int", "15"]),
            ("defaults", &["write", "NSGlobalDomain", "ApplePressAndHoldEnabled", "-bool", "false"]),
            ("defaults", &["write", "com.apple.finder", "AppleShowAllFiles", "-bool", "true"]),
            ("defaults", &["write", "com.apple.finder", "ShowPathbar", "-bool", "true"]),
            ("defaults", &["write", "com.apple.finder", "ShowStatusBar", "-bool", "true"]),
            ("defaults", &["write", "NSGlobalDomain", "AppleShowAllExtensions", "-bool", "true"]),
            ("defaults", &["write", "NSGlobalDomain", "NSAutomaticSpellingCorrectionEnabled", "-bool", "false"]),
            ("defaults", &["write", "NSGlobalDomain", "NSAutomaticQuoteSubstitutionEnabled", "-bool", "false"]),
            ("defaults", &["write", "NSGlobalDomain", "NSAutomaticDashSubstitutionEnabled", "-bool", "false"]),
        ];

        for (prog, args) in commands {
            if dry_run {
                let cmd_str = std::iter::once(*prog)
                    .chain(args.iter().copied())
                    .collect::<Vec<_>>()
                    .join(" ");
                let _ = tx
                    .send(InstallerEvent::LogLine {
                        step_id: step_id.to_string(),
                        line: format!("[dry-run] {cmd_str}"),
                    })
                    .await;
                continue;
            }

            run_command(prog, args, step_id, &tx).await?;
        }

        if !dry_run {
            // Restart Finder
            let _ = tx
                .send(InstallerEvent::LogLine {
                    step_id: step_id.to_string(),
                    line: "restarting Finder...".to_string(),
                })
                .await;
            let _ = tokio::process::Command::new("killall")
                .arg("Finder")
                .status()
                .await;
        }

        Ok(())
    }
}
