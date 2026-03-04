use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::super::app::App;
use super::theme;

const FIELDS: [(&str, &str); 5] = [
    ("git name    ", "Your Name"),
    ("work email  ", "user@work.example.com"),
    ("gpg key     ", "GPG key ID for commit signing (leave blank to skip)"),
    ("personal email", "user@personal.example.com"),
    ("hostname    ", "macbook-name"),
];

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let constraints: Vec<Constraint> = std::iter::repeat(Constraint::Length(3))
        .take(FIELDS.len())
        .chain([Constraint::Min(0), Constraint::Length(1)])
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, ((label, placeholder), value)) in
        FIELDS.iter().zip(app.config_fields.iter()).enumerate()
    {
        let focused = app.config_cursor == i;
        let border_style = if focused {
            theme::border_active()
        } else {
            theme::border_inactive()
        };

        let display = if value.is_empty() {
            Span::styled(*placeholder, theme::muted())
        } else {
            Span::styled(value.as_str(), theme::bright())
        };

        let cursor = if focused { Span::styled("_", theme::accent()) } else { Span::raw("") };

        let content = Paragraph::new(Line::from(vec![
            Span::styled("  ", theme::muted()),
            display,
            cursor,
        ]))
        .block(
            Block::default()
                .title(format!(" {label} "))
                .borders(Borders::ALL)
                .border_style(border_style)
                .title_style(if focused {
                    theme::accent().add_modifier(Modifier::BOLD)
                } else {
                    theme::muted()
                }),
        );
        f.render_widget(content, chunks[i]);
    }

    let hints = Paragraph::new(Line::from(vec![
        Span::styled("tab/enter", theme::accent()),
        Span::styled(" next field  ", theme::key_hint()),
        Span::styled("shift+tab", theme::accent()),
        Span::styled(" prev  ", theme::key_hint()),
        Span::styled("enter", theme::accent()),
        Span::styled(" on last field to continue", theme::key_hint()),
    ]));
    f.render_widget(hints, *chunks.last().unwrap());
}
