pub mod theme;

mod confirm;
mod config;
mod done;
mod progress;
mod selection;
mod welcome;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Modifier,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use super::app::{App, Screen};
use theme as t;

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    // Outer chrome: title bar + content + status bar
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title bar
            Constraint::Min(0),    // content
        ])
        .split(size);

    // Title bar
    let screen_name = match app.screen {
        Screen::Welcome => "welcome",
        Screen::Selection => "select modules",
        Screen::Config => "configure",
        Screen::Confirm => "confirm",
        Screen::Progress => "installing",
        Screen::Done => "done",
    };
    let dry_run_tag = if app.dry_run {
        Span::styled(" [dry-run]", t::accent())
    } else {
        Span::raw("")
    };
    let title_bar = Paragraph::new(Line::from(vec![
        Span::styled("dev-setup", t::accent().add_modifier(Modifier::BOLD)),
        Span::styled("  /  ", t::muted()),
        Span::styled(screen_name, t::bright()),
        dry_run_tag,
    ]));
    f.render_widget(title_bar, root[0]);

    // Content area with padding
    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(root[1]);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(content[1]);

    let area = inner[1];

    match app.screen {
        Screen::Welcome => welcome::render(f, app, area),
        Screen::Selection => selection::render(f, app, area),
        Screen::Config => config::render(f, app, area),
        Screen::Confirm => confirm::render(f, app, area),
        Screen::Progress => progress::render(f, app, area),
        Screen::Done => done::render(f, app, area),
    }
}
