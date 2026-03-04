use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::super::app::App;
use super::theme;

const MODULE_LABELS: [(&str, &str); 4] = [
    ("homebrew", "homebrew packages  — brew bundle install"),
    ("dotfiles", "dotfiles symlinks  — git, zsh, tmux, ghostty, nvim"),
    ("macos",    "macos defaults     — key repeat, Finder, autocorrect"),
    ("post",     "post-install       — omz, tpm, fnm+node, gh-dash, colima"),
];

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // modules
            Constraint::Length(7),  // config
            Constraint::Length(2),  // dry-run flag
            Constraint::Min(0),
            Constraint::Length(1),  // key hints
        ])
        .split(area);

    // Modules to run
    let selected_modules = app.selection_modules();
    let module_items: Vec<ListItem> = MODULE_LABELS
        .iter()
        .filter(|(id, _)| selected_modules.contains(&id.to_string()))
        .map(|(_, label)| {
            ListItem::new(Line::from(vec![
                Span::styled("  [+] ", theme::success()),
                Span::styled(*label, theme::normal()),
            ]))
        })
        .collect();

    let modules_list = List::new(module_items).block(
        Block::default()
            .title(" will run ")
            .borders(Borders::ALL)
            .border_style(theme::border_inactive()),
    );
    f.render_widget(modules_list, chunks[0]);

    // Config summary
    let cfg = &app.state.config;
    let config_lines = vec![
        Line::from(vec![
            Span::styled("  git name    ", theme::muted()),
            Span::styled(
                if cfg.git_name.is_empty() { "(blank)" } else { &cfg.git_name },
                theme::normal(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  work email  ", theme::muted()),
            Span::styled(
                if cfg.work_email.is_empty() { "(blank)" } else { &cfg.work_email },
                theme::normal(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  gpg key     ", theme::muted()),
            Span::styled(
                if cfg.gpg_key.is_empty() { "(none)" } else { &cfg.gpg_key },
                theme::normal(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  hostname    ", theme::muted()),
            Span::styled(
                if cfg.hostname.is_empty() { "(blank)" } else { &cfg.hostname },
                theme::normal(),
            ),
        ]),
    ];
    let config_block = Paragraph::new(config_lines).block(
        Block::default()
            .title(" config ")
            .borders(Borders::ALL)
            .border_style(theme::border_inactive()),
    );
    f.render_widget(config_block, chunks[1]);

    // Dry-run flag
    let dry_run_line = if app.dry_run {
        Paragraph::new(Line::from(vec![
            Span::styled("  ", theme::muted()),
            Span::styled("--dry-run", theme::accent().add_modifier(Modifier::BOLD)),
            Span::styled(" — commands will be printed, not executed", theme::muted()),
        ]))
    } else {
        Paragraph::new(Line::from(Span::styled(
            "  ready to install",
            theme::muted(),
        )))
    };
    f.render_widget(dry_run_line, chunks[2]);

    // Key hints
    let hints = Paragraph::new(Line::from(vec![
        Span::styled("enter", theme::accent()),
        Span::styled(" / ", theme::key_hint()),
        Span::styled("y", theme::accent()),
        Span::styled(" run  ", theme::key_hint()),
        Span::styled("n", theme::accent()),
        Span::styled(" back  ", theme::key_hint()),
        Span::styled("q", theme::accent()),
        Span::styled(" quit", theme::key_hint()),
    ]));
    f.render_widget(hints, chunks[4]);
}
