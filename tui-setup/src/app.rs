use color_eyre::Result;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyModifiers};
use futures::StreamExt;
use ratatui::{backend::Backend, Terminal};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::{sync::mpsc, time::interval};

use crate::{
    installer::{self, InstallerEvent},
    state::{InstallState, StepState, StepStatus, UserConfig},
    ui,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Welcome,
    Selection,
    Config,
    Confirm,
    Progress,
    Done,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub dry_run: bool,
    pub state: InstallState,
    pub dotfiles_dir: String,

    // Selection screen
    pub selection: [bool; 4],
    pub selection_cursor: usize,

    // Config screen — matches [git_name, work_email, gpg_key, personal_email, hostname]
    pub config_fields: [String; 5],
    pub config_cursor: usize,

    // Progress screen
    pub step_logs: Vec<(String, Vec<String>)>, // (step_id, lines)
    pub current_step_id: Option<String>,
    pub log_offset: usize,
    pub manual_scroll: bool,
    pub install_done: bool,
    pub step_start_times: HashMap<String, Instant>,
    pub step_elapsed: HashMap<String, u64>,

    // Installer channel (stored temporarily until picked up by run())
    pub pending_installer_rx: Option<mpsc::Receiver<InstallerEvent>>,
}

impl App {
    pub fn new(dry_run: bool) -> Result<Self> {
        let state = InstallState::load();

        // Detect dotfiles dir: prefer CWD (user runs from repo root)
        let dotfiles_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .to_string_lossy()
            .into_owned();

        // Pre-populate config fields from saved state
        let config_fields = [
            state.config.git_name.clone(),
            state.config.work_email.clone(),
            state.config.gpg_key.clone(),
            state.config.personal_email.clone(),
            state.config.hostname.clone(),
        ];

        Ok(Self {
            screen: Screen::Welcome,
            should_quit: false,
            dry_run,
            state,
            dotfiles_dir,
            selection: [true, true, true, true],
            selection_cursor: 0,
            config_fields,
            config_cursor: 0,
            step_logs: Vec::new(),
            current_step_id: None,
            log_offset: 0,
            manual_scroll: false,
            install_done: false,
            step_start_times: HashMap::new(),
            step_elapsed: HashMap::new(),
            pending_installer_rx: None,
        })
    }

    pub fn selection_modules(&self) -> Vec<String> {
        let names = ["homebrew", "dotfiles", "macos", "post"];
        names
            .iter()
            .enumerate()
            .filter(|(i, _)| self.selection[*i])
            .map(|(_, n)| n.to_string())
            .collect()
    }

    fn start_installer(&mut self) {
        let (tx, rx) = mpsc::channel(512);
        self.pending_installer_rx = Some(rx);

        let modules = self.selection_modules();
        let dotfiles_dir = self.dotfiles_dir.clone();
        let dry_run = self.dry_run;

        // Initialise step states + log buffers
        let steps = installer::build_steps(&modules, &dotfiles_dir);
        self.step_logs.clear();
        for step in &steps {
            let id = step.id().to_string();
            self.state
                .steps
                .insert(id.clone(), StepState::new(step.id(), step.label()));
            self.step_logs.push((id, Vec::new()));
        }
        let _ = self.state.save();

        tokio::spawn(async move {
            installer::run_installer(steps, tx, dry_run).await;
        });
    }

    fn handle_installer_event(&mut self, event: InstallerEvent) {
        match event {
            InstallerEvent::StepStarted { step_id } => {
                if let Some(step) = self.state.steps.get_mut(&step_id) {
                    step.status = StepStatus::Running;
                }
                self.current_step_id = Some(step_id.clone());
                self.step_start_times.insert(step_id, Instant::now());
                self.log_offset = 0;
                self.manual_scroll = false;
                let _ = self.state.save();
            }
            InstallerEvent::LogLine { step_id, line } => {
                if let Some((_, lines)) = self
                    .step_logs
                    .iter_mut()
                    .find(|(id, _)| *id == step_id)
                {
                    lines.push(line);
                }
            }
            InstallerEvent::StepCompleted { step_id, success } => {
                if let Some(step) = self.state.steps.get_mut(&step_id) {
                    step.status = StepStatus::Done { success };
                    if let Some(start) = self.step_start_times.get(&step_id) {
                        step.elapsed_secs = start.elapsed().as_secs();
                        self.step_elapsed
                            .insert(step_id.clone(), step.elapsed_secs);
                    }
                }
                let _ = self.state.save();
            }
            InstallerEvent::AllDone => {
                self.install_done = true;
            }
        }
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut ct_stream = EventStream::new();
        let mut tick_timer = interval(Duration::from_millis(500));
        let mut installer_rx: Option<mpsc::Receiver<InstallerEvent>> = None;

        loop {
            // Pick up a newly started installer channel
            if let Some(rx) = self.pending_installer_rx.take() {
                installer_rx = Some(rx);
            }

            terminal.draw(|f| ui::render(f, self))?;

            tokio::select! {
                maybe_event = ct_stream.next() => {
                    match maybe_event {
                        Some(Ok(CrosstermEvent::Key(key))) => self.handle_key(key),
                        Some(Err(_)) | None => break,
                        _ => {}
                    }
                }
                maybe_installer = recv_installer(&mut installer_rx) => {
                    if let Some(event) = maybe_installer {
                        self.handle_installer_event(event);
                        if self.install_done {
                            self.screen = Screen::Done;
                        }
                    }
                }
                _ = tick_timer.tick() => {
                    // Update live elapsed timers for running steps
                    for (step_id, start) in &self.step_start_times {
                        if let Some(step) = self.state.steps.get(step_id) {
                            if step.status == StepStatus::Running {
                                self.step_elapsed.insert(step_id.clone(), start.elapsed().as_secs());
                            }
                        }
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        // Global: Ctrl-C always quits
        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
            self.should_quit = true;
            return;
        }

        // Global: 'q' quits except during active install
        if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::NONE {
            if self.screen != Screen::Progress || self.install_done {
                self.should_quit = true;
                return;
            }
        }

        match &self.screen {
            Screen::Welcome => self.handle_welcome_key(key),
            Screen::Selection => self.handle_selection_key(key),
            Screen::Config => self.handle_config_key(key),
            Screen::Confirm => self.handle_confirm_key(key),
            Screen::Progress => self.handle_progress_key(key),
            Screen::Done => self.handle_done_key(key),
        }
    }

    fn handle_welcome_key(&mut self, key: crossterm::event::KeyEvent) {
        if key.code == KeyCode::Enter {
            self.screen = Screen::Selection;
        }
    }

    fn handle_selection_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.selection_cursor = self.selection_cursor.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selection_cursor < 3 {
                    self.selection_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                self.selection[self.selection_cursor] = !self.selection[self.selection_cursor];
            }
            KeyCode::Enter => {
                if self.selection.iter().any(|&s| s) {
                    self.screen = Screen::Config;
                }
            }
            _ => {}
        }
    }

    fn handle_config_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::Down => {
                self.config_cursor = (self.config_cursor + 1) % 5;
            }
            KeyCode::BackTab | KeyCode::Up => {
                self.config_cursor = if self.config_cursor == 0 {
                    4
                } else {
                    self.config_cursor - 1
                };
            }
            KeyCode::Backspace => {
                self.config_fields[self.config_cursor].pop();
            }
            KeyCode::Char(c) => {
                self.config_fields[self.config_cursor].push(c);
            }
            KeyCode::Enter => {
                if self.config_cursor < 4 {
                    self.config_cursor += 1;
                } else {
                    self.commit_config();
                    self.screen = Screen::Confirm;
                }
            }
            _ => {}
        }
    }

    fn commit_config(&mut self) {
        self.state.config = UserConfig {
            git_name: self.config_fields[0].clone(),
            work_email: self.config_fields[1].clone(),
            gpg_key: self.config_fields[2].clone(),
            personal_email: self.config_fields[3].clone(),
            hostname: self.config_fields[4].clone(),
        };
    }

    fn handle_confirm_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char('y') => {
                self.state.selected_modules = self.selection_modules();
                self.commit_config();
                let _ = self.state.save();
                self.apply_git_config();
                self.start_installer();
                self.screen = Screen::Progress;
            }
            KeyCode::Char('n') => {
                self.screen = Screen::Config;
            }
            _ => {}
        }
    }

    fn apply_git_config(&self) {
        let cfg = &self.state.config;

        let pairs = [
            ("user.name", cfg.git_name.as_str()),
            ("user.email", cfg.work_email.as_str()),
            ("user.signingkey", cfg.gpg_key.as_str()),
        ];

        for (key, value) in &pairs {
            if !value.is_empty() {
                let _ = std::process::Command::new("git")
                    .args(["config", "--global", key, value])
                    .status();
            }
        }

        if !cfg.hostname.is_empty() {
            let _ = std::process::Command::new("scutil")
                .args(["--set", "HostName", &cfg.hostname])
                .status();
            let _ = std::process::Command::new("scutil")
                .args(["--set", "ComputerName", &cfg.hostname])
                .status();
        }
    }

    fn handle_progress_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.manual_scroll = true;
                self.log_offset = self.log_offset.saturating_add(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.manual_scroll = true;
                self.log_offset = self.log_offset.saturating_sub(1);
            }
            KeyCode::Char('g') => {
                self.manual_scroll = false;
                self.log_offset = 0;
            }
            _ => {}
        }
    }

    fn handle_done_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char('q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}

/// Async helper: receive from installer channel, or pend forever if None.
async fn recv_installer(
    rx: &mut Option<mpsc::Receiver<InstallerEvent>>,
) -> Option<InstallerEvent> {
    match rx {
        Some(r) => r.recv().await,
        None => std::future::pending().await,
    }
}
