use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::super::app::App;
use super::theme;
use crate::state::StepStatus;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let any_failed = app
        .state
        .steps
        .values()
        .any(|s| matches!(s.status, StepStatus::Done { success: false }));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // status
            Constraint::Length(8),  // step summary
            Constraint::Length(8),  // manual steps
            Constraint::Min(0),
            Constraint::Length(1),  // hints
        ])
        .split(area);

    // Status line
    let (status_text, status_style) = if any_failed {
        (
            "  installation completed with errors",
            theme::err().add_modifier(Modifier::BOLD),
        )
    } else {
        (
            "  installation complete",
            theme::success().add_modifier(Modifier::BOLD),
        )
    };
    let status = Paragraph::new(Line::from(Span::styled(status_text, status_style))).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(theme::border_inactive()),
    );
    f.render_widget(status, chunks[0]);

    // Step summary
    let step_items: Vec<ListItem> = app
        .state
        .steps
        .values()
        .map(|step| {
            let (marker, style) = match &step.status {
                StepStatus::Done { success: true } => ("[+]", theme::success()),
                StepStatus::Done { success: false } => ("[!]", theme::err()),
                _ => ("[ ]", theme::muted()),
            };
            ListItem::new(Line::from(vec![
                Span::styled(marker, style),
                Span::raw("  "),
                Span::styled(&step.label, theme::normal()),
            ]))
        })
        .collect();

    let step_list = List::new(step_items).block(
        Block::default()
            .title(" results ")
            .borders(Borders::ALL)
            .border_style(theme::border_inactive()),
    );
    f.render_widget(step_list, chunks[1]);

    // Manual steps
    let manual_items = vec![
        ListItem::new(Line::from(vec![
            Span::styled("  1. ", theme::muted()),
            Span::styled("restart terminal", theme::normal()),
            Span::styled(" (reload zsh + paths)", theme::muted()),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  2. ", theme::muted()),
            Span::styled("install tmux plugins", theme::normal()),
            Span::styled("   prefix + I  (inside tmux)", theme::muted()),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  3. ", theme::muted()),
            Span::styled("open nvim", theme::normal()),
            Span::styled("                plugins install automatically", theme::muted()),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  4. ", theme::muted()),
            Span::styled("import GPG key", theme::normal()),
            Span::styled("           gpg --import <keyfile>", theme::muted()),
        ])),
    ];

    let manual_list = List::new(manual_items).block(
        Block::default()
            .title(" manual steps remaining ")
            .borders(Borders::ALL)
            .border_style(theme::border_inactive()),
    );
    f.render_widget(manual_list, chunks[2]);

    // Hints
    let hints = Paragraph::new(Line::from(vec![
        Span::styled("enter", theme::accent()),
        Span::styled(" / ", theme::key_hint()),
        Span::styled("q", theme::accent()),
        Span::styled(" exit", theme::key_hint()),
    ]));
    f.render_widget(hints, chunks[4]);
}
