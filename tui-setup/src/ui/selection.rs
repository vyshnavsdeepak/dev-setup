use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::super::app::App;
use super::theme;

const MODULES: [(&str, &str, &str); 4] = [
    (
        "homebrew",
        "PACKAGES",
        "brew bundle install from Brewfile",
    ),
    (
        "dotfiles",
        "DOTFILES",
        "symlink git, zsh, tmux, ghostty, nvim configs",
    ),
    (
        "macos",
        "SYSTEM",
        "defaults write for key repeat, Finder, autocorrect",
    ),
    (
        "post",
        "POST-INSTALL",
        "oh-my-zsh, tpm, fnm+node, gh-dash, colima",
    ),
];

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // header
            Constraint::Min(6),     // list
            Constraint::Length(1),  // key hints
        ])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled("select modules to install", theme::muted()),
    ]));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = MODULES
        .iter()
        .enumerate()
        .map(|(i, (_, group, desc))| {
            let checked = app.selection[i];
            let focused = app.selection_cursor == i;

            let check_char = if checked { "+" } else { " " };
            let check_style = if checked {
                theme::success()
            } else {
                theme::muted()
            };

            let bracket_style = if focused {
                theme::accent()
            } else {
                theme::border_inactive()
            };

            let label_style = if focused {
                theme::bright()
            } else if checked {
                theme::normal()
            } else {
                theme::muted()
            };

            let desc_style = theme::muted();

            ListItem::new(Line::from(vec![
                Span::styled("[", bracket_style),
                Span::styled(check_char, check_style),
                Span::styled("] ", bracket_style),
                Span::styled(
                    format!("{:<14}", group),
                    label_style.add_modifier(if focused { Modifier::BOLD } else { Modifier::empty() }),
                ),
                Span::styled(*desc, desc_style),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_active()),
    );
    f.render_widget(list, chunks[1]);

    let hints = Paragraph::new(Line::from(vec![
        Span::styled("j/k", theme::accent()),
        Span::styled(" move  ", theme::key_hint()),
        Span::styled("space", theme::accent()),
        Span::styled(" toggle  ", theme::key_hint()),
        Span::styled("enter", theme::accent()),
        Span::styled(" confirm  ", theme::key_hint()),
        Span::styled("q", theme::accent()),
        Span::styled(" quit", theme::key_hint()),
    ]));
    f.render_widget(hints, chunks[2]);
}
