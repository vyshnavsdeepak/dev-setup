use color_eyre::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Pending,
    Running,
    Done { success: bool },
}

impl Default for StepStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepState {
    pub id: String,
    pub label: String,
    pub status: StepStatus,
    pub elapsed_secs: u64,
}

impl StepState {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            status: StepStatus::Pending,
            elapsed_secs: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub git_name: String,
    pub work_email: String,
    pub gpg_key: String,
    pub personal_email: String,
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InstallState {
    pub steps: IndexMap<String, StepState>,
    pub config: UserConfig,
    pub selected_modules: Vec<String>,
}

impl InstallState {
    pub fn state_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| {
                PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".config")
            })
            .join("tui-setup")
            .join("state.json")
    }

    pub fn load() -> Self {
        let path = Self::state_path();
        if path.exists() {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if let Ok(mut state) = serde_json::from_str::<Self>(&contents) {
                    // Reset Running → Pending (from a crashed run)
                    for step in state.steps.values_mut() {
                        if step.status == StepStatus::Running {
                            step.status = StepStatus::Pending;
                        }
                    }
                    return state;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::state_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    pub fn has_prior_state(&self) -> bool {
        self.steps
            .values()
            .any(|s| matches!(s.status, StepStatus::Done { success: true }))
    }
}
