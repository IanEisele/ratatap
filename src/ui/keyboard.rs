use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::collections::HashMap;

use crate::stats::CharStats;
use crate::ui::theme::{current_key_style, key_error_color, subtitle_style, Theme};

/// Get the style for a key based on whether it's current and its error history
fn get_key_style(key: char, is_current: bool, char_stats: &HashMap<char, CharStats>, theme: &Theme) -> Style {
    if is_current {
        current_key_style(theme)
    } else {
        let error_count = char_stats.get(&key).map(|s| s.total_errors).unwrap_or(0);
        Style::default().fg(key_error_color(theme, error_count))
    }
}

pub fn render_keyboard(
    f: &mut Frame,
    area: Rect,
    current_char: Option<char>,
    char_stats: &HashMap<char, CharStats>,
    theme: &Theme,
) {
    let keys = [
        vec!['`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '='],
        vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\\'],
        vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\''],
        vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
    ];

    let mut lines = Vec::new();

    for row in &keys {
        let spans: Vec<Span> = row
            .iter()
            .map(|&key| {
                let is_current = current_char == Some(key);
                let style = get_key_style(key, is_current, char_stats, theme);
                Span::styled(format!(" {} ", key.to_uppercase()), style)
            })
            .collect();
        lines.push(Line::from(spans));
    }

    // Space bar
    let space_style = get_key_style(' ', current_char == Some(' '), char_stats, theme);
    lines.push(Line::from(vec![Span::styled("     [ SPACE ]     ", space_style)]));

    let keyboard = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.primary()))
                .title("Keyboard")
                .title_style(subtitle_style(theme)),
        )
        .alignment(Alignment::Center);

    f.render_widget(keyboard, area);
}
