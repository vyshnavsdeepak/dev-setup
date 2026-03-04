use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::super::app::App;
use super::theme;
use crate::state::StepStatus;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(area);

    render_step_list(f, app, chunks[0]);
    render_log_panel(f, app, chunks[1]);

    // Key hints
    let scroll_hint = if app.manual_scroll {
        "auto-scroll off  "
    } else {
        "auto-scroll on   "
    };
    let hints = Paragraph::new(Line::from(vec![
        Span::styled("j/k", theme::accent()),
        Span::styled(" scroll  ", theme::key_hint()),
        Span::styled("g", theme::accent()),
        Span::styled(" bottom  ", theme::key_hint()),
        Span::styled(scroll_hint, theme::muted()),
    ]));
    f.render_widget(hints, chunks[2]);
}

fn render_step_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .state
        .steps
        .values()
        .map(|step| {
            let (bracket, bracket_style) = match &step.status {
                StepStatus::Pending => ("[ ]", theme::muted()),
                StepStatus::Running => ("[>]", theme::accent()),
                StepStatus::Done { success: true } => {
                    if app.step_logs.iter().any(|(id, lines)| {
                        id == &step.id
                            && lines.iter().any(|l| l.starts_with("[skip]"))
                    }) {
                        ("[-]", theme::muted())
                    } else {
                        ("[+]", theme::success())
                    }
                }
                StepStatus::Done { success: false } => ("[!]", theme::err()),
            };

            let elapsed = app
                .step_elapsed
                .get(&step.id)
                .copied()
                .or({
                    if matches!(step.status, StepStatus::Done { .. }) {
                        Some(step.elapsed_secs)
                    } else {
                        None
                    }
                });

            let elapsed_str = match elapsed {
                Some(s) if s > 0 => format!(" ({s}s)"),
                _ => String::new(),
            };

            ListItem::new(Line::from(vec![
                Span::styled(bracket, bracket_style),
                Span::raw(" "),
                Span::styled(&step.label, theme::normal()),
                Span::styled(elapsed_str, theme::muted()),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" installation ")
            .borders(Borders::ALL)
            .border_style(theme::border_active()),
    );
    f.render_widget(list, area);
}

fn render_log_panel(f: &mut Frame, app: &App, area: Rect) {
    let current_id = app.current_step_id.as_deref().unwrap_or("");

    let log_lines: Vec<Line> = app
        .step_logs
        .iter()
        .find(|(id, _)| id == current_id)
        .map(|(_, lines)| {
            lines
                .iter()
                .map(|l| {
                    let style = if l.starts_with("[error]") || l.starts_with("[!]") {
                        theme::err()
                    } else if l.starts_with("[skip]") || l.starts_with("[ok]") {
                        theme::muted()
                    } else if l.starts_with("[dry-run]") {
                        theme::accent()
                    } else {
                        theme::normal()
                    };
                    Line::from(Span::styled(l.as_str(), style))
                })
                .collect()
        })
        .unwrap_or_default();

    let total_lines = log_lines.len();
    let visible_height = area.height.saturating_sub(2) as usize; // minus borders

    let scroll_offset = if app.manual_scroll {
        app.log_offset.min(total_lines.saturating_sub(visible_height))
    } else {
        // Auto-scroll: show tail
        total_lines.saturating_sub(visible_height)
    };

    let title = format!(
        " output: {current_id} {} ",
        if app.manual_scroll { "[j/k scroll]" } else { "[auto]" }
    );

    let log_widget = Paragraph::new(log_lines)
        .scroll((scroll_offset as u16, 0))
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(theme::border_inactive()),
        );
    f.render_widget(log_widget, area);
}
