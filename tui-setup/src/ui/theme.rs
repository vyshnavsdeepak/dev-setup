// Industrial minimal palette — extracted from ghostty/config
use ratatui::style::{Color, Modifier, Style};

#[allow(dead_code)]
pub const BG: Color = Color::Rgb(0x19, 0x19, 0x19);
pub const FG: Color = Color::Rgb(0xc0, 0xc0, 0xc0);
pub const BORDER: Color = Color::Rgb(0x33, 0x33, 0x33);
pub const MUTED: Color = Color::Rgb(0x60, 0x60, 0x60);
pub const ACCENT: Color = Color::Rgb(0xcc, 0x78, 0x32); // amber
pub const BRIGHT: Color = Color::Rgb(0xe0, 0xe0, 0xe0);
pub const ERR: Color = Color::Rgb(0xa0, 0x40, 0x40);
pub const SUCCESS: Color = Color::Rgb(0x60, 0x80, 0x40);

pub fn normal() -> Style {
    Style::default().fg(FG)
}

pub fn muted() -> Style {
    Style::default().fg(MUTED)
}

pub fn accent() -> Style {
    Style::default().fg(ACCENT)
}

pub fn bright() -> Style {
    Style::default().fg(BRIGHT).add_modifier(Modifier::BOLD)
}

pub fn err() -> Style {
    Style::default().fg(ERR)
}

pub fn success() -> Style {
    Style::default().fg(SUCCESS)
}

pub fn border_active() -> Style {
    Style::default().fg(ACCENT)
}

pub fn border_inactive() -> Style {
    Style::default().fg(BORDER)
}

pub fn key_hint() -> Style {
    Style::default().fg(MUTED)
}
