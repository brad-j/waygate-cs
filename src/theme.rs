//! Palette. Everything is explicit RGB so fades interpolate cleanly.

use ratatui::style::{Color, Modifier, Style};

pub const ACCENT: Color = Color::Rgb(217, 119, 87);
pub const TEXT: Color = Color::Rgb(212, 212, 206);
pub const BRIGHT: Color = Color::Rgb(242, 242, 236);
pub const DIM: Color = Color::Rgb(140, 140, 150);
pub const FAINT: Color = Color::Rgb(74, 74, 84);
pub const SEL_BG: Color = Color::Rgb(44, 44, 56);
pub const HOVER_BG: Color = Color::Rgb(32, 32, 41);
pub const MENU_BG: Color = Color::Rgb(26, 26, 33);
pub const WAIT: Color = Color::Rgb(232, 184, 104);
pub const LIVE: Color = Color::Rgb(126, 214, 142);
pub const LIVE_DIM: Color = Color::Rgb(46, 92, 58);
pub const CODE: Color = Color::Rgb(146, 190, 246);
pub const ERR: Color = Color::Rgb(236, 112, 112);
pub const FADE_FROM: Color = Color::Rgb(34, 34, 42);

pub fn text() -> Style {
    Style::new().fg(TEXT)
}
pub fn dim() -> Style {
    Style::new().fg(DIM)
}
pub fn faint() -> Style {
    Style::new().fg(FAINT)
}
pub fn bright() -> Style {
    Style::new().fg(BRIGHT).add_modifier(Modifier::BOLD)
}
pub fn accent() -> Style {
    Style::new().fg(ACCENT)
}
pub fn label() -> Style {
    Style::new().fg(ACCENT).add_modifier(Modifier::BOLD)
}
pub fn code() -> Style {
    Style::new().fg(CODE)
}
pub fn matched() -> Style {
    Style::new()
        .fg(ACCENT)
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
}

pub fn lerp(a: Color, b: Color, t: f32) -> Color {
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t.clamp(0.0, 1.0)) as u8;
            Color::Rgb(m(r1, r2), m(g1, g2), m(b1, b2))
        }
        _ => b,
    }
}
