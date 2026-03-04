use async_trait::async_trait;
use color_eyre::Result;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::mpsc,
};
use std::process::Stdio;

pub mod dotfiles;
pub mod homebrew;
pub mod macos;
pub mod post;

use dotfiles::DotfilesStep;
use homebrew::HomebrewStep;
use macos::MacosStep;
use post::PostInstallStep;


#[derive(Debug, Clone)]
pub enum InstallerEvent {
    LogLine { step_id: String, line: String },
    StepStarted { step_id: String },
    StepCompleted { step_id: String, success: bool },
    AllDone,
}

#[async_trait]
#[allow(dead_code)]
pub trait InstallStep: Send + Sync {
    fn id(&self) -> &str;
    fn label(&self) -> &str;
    fn estimated_secs(&self) -> u64;
    async fn is_already_done(&self) -> bool;
    async fn execute(&self, tx: mpsc::Sender<InstallerEvent>, dry_run: bool) -> Result<()>;
}

/// Stream a command's stdout+stderr to the log channel. Returns true on success.
pub async fn run_command(
    program: &str,
    args: &[&str],
    step_id: &str,
    tx: &mpsc::Sender<InstallerEvent>,
) -> Result<bool> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("piped stdout");
    let stderr = child.stderr.take().expect("piped stderr");

    let tx1 = tx.clone();
    let id1 = step_id.to_string();
    let stdout_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let _ = tx1
                .send(InstallerEvent::LogLine {
                    step_id: id1.clone(),
                    line,
                })
                .await;
        }
    });

    let tx2 = tx.clone();
    let id2 = step_id.to_string();
    let stderr_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let _ = tx2
                .send(InstallerEvent::LogLine {
                    step_id: id2.clone(),
                    line,
                })
                .await;
        }
    });

    let _ = tokio::join!(stdout_task, stderr_task);
    let status = child.wait().await?;
    Ok(status.success())
}

/// Check if a command exits successfully (for idempotency checks).
pub async fn check_command(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn build_steps(selected_modules: &[String], dotfiles_dir: &str) -> Vec<Box<dyn InstallStep>> {
    let mut steps: Vec<Box<dyn InstallStep>> = Vec::new();

    if selected_modules.contains(&"homebrew".to_string()) {
        steps.push(Box::new(HomebrewStep::new(dotfiles_dir)));
    }
    if selected_modules.contains(&"dotfiles".to_string()) {
        steps.push(Box::new(DotfilesStep::new(dotfiles_dir)));
    }
    if selected_modules.contains(&"macos".to_string()) {
        steps.push(Box::new(MacosStep));
    }
    if selected_modules.contains(&"post".to_string()) {
        steps.push(Box::new(PostInstallStep::new(dotfiles_dir)));
    }

    steps
}

pub async fn run_installer(
    steps: Vec<Box<dyn InstallStep>>,
    tx: mpsc::Sender<InstallerEvent>,
    dry_run: bool,
) {
    for step in &steps {
        let _ = tx
            .send(InstallerEvent::StepStarted {
                step_id: step.id().to_string(),
            })
            .await;

        if step.is_already_done().await {
            let _ = tx
                .send(InstallerEvent::LogLine {
                    step_id: step.id().to_string(),
                    line: format!("[skip] {} already configured", step.label()),
                })
                .await;
            let _ = tx
                .send(InstallerEvent::StepCompleted {
                    step_id: step.id().to_string(),
                    success: true,
                })
                .await;
            continue;
        }

        match step.execute(tx.clone(), dry_run).await {
            Ok(_) => {
                let _ = tx
                    .send(InstallerEvent::StepCompleted {
                        step_id: step.id().to_string(),
                        success: true,
                    })
                    .await;
            }
            Err(e) => {
                let _ = tx
                    .send(InstallerEvent::LogLine {
                        step_id: step.id().to_string(),
                        line: format!("[error] {e}"),
                    })
                    .await;
                let _ = tx
                    .send(InstallerEvent::StepCompleted {
                        step_id: step.id().to_string(),
                        success: false,
                    })
                    .await;
            }
        }
    }

    let _ = tx.send(InstallerEvent::AllDone).await;
}
