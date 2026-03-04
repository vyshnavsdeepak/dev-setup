use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use sysinfo::System;

use super::super::app::App;
use super::theme;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let ram_gb = sys.total_memory() / 1_073_741_824;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // title
            Constraint::Length(5),  // machine info
            Constraint::Length(3),  // resume notice
            Constraint::Min(0),     // spacer
            Constraint::Length(1),  // key hints
        ])
        .split(area);

    // Title
    let title = Paragraph::new(Line::from(vec![
        Span::styled("dev-setup", theme::accent().add_modifier(Modifier::BOLD)),
        Span::styled(" / interactive installer", theme::muted()),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(theme::border_inactive()),
    );
    f.render_widget(title, chunks[0]);

    // Machine info
    let info_lines = vec![
        Line::from(vec![
            Span::styled("  host  ", theme::muted()),
            Span::styled(&hostname, theme::bright()),
        ]),
        Line::from(vec![
            Span::styled("  cpu   ", theme::muted()),
            Span::styled(&cpu_brand, theme::normal()),
        ]),
        Line::from(vec![
            Span::styled("  ram   ", theme::muted()),
            Span::styled(format!("{ram_gb} GB"), theme::normal()),
        ]),
    ];
    let info = Paragraph::new(info_lines).block(
        Block::default()
            .title(" machine ")
            .borders(Borders::ALL)
            .border_style(theme::border_inactive()),
    );
    f.render_widget(info, chunks[1]);

    // Resume notice
    let notice = if app.state.has_prior_state() {
        Paragraph::new(Line::from(vec![
            Span::styled("  prior state found — ", theme::muted()),
            Span::styled("completed steps will be skipped", theme::accent()),
        ]))
    } else if app.dry_run {
        Paragraph::new(Line::from(vec![
            Span::styled("  ", theme::muted()),
            Span::styled("--dry-run", theme::accent()),
            Span::styled(" mode — no changes will be made", theme::muted()),
        ]))
    } else {
        Paragraph::new(Line::from(Span::styled(
            "  fresh install",
            theme::muted(),
        )))
    };
    f.render_widget(notice, chunks[2]);

    // Key hints
    let hints = Paragraph::new(Line::from(vec![
        Span::styled("enter", theme::accent()),
        Span::styled(" continue  ", theme::key_hint()),
        Span::styled("q", theme::accent()),
        Span::styled(" quit", theme::key_hint()),
    ]));
    f.render_widget(hints, chunks[4]);
}
