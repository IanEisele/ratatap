use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};

use crate::ui::theme::{primary_style, progress_bar_style, subtitle_style, Theme};

pub fn render_wpm_sparkline(f: &mut Frame, area: Rect, wpm_history: &[u64], theme: &Theme) {
    if wpm_history.is_empty() {
        let placeholder = Paragraph::new("No history yet")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme.primary()))
                    .title("WPM History")
                    .title_style(subtitle_style(theme)),
            )
            .style(subtitle_style(theme))
            .alignment(Alignment::Center);
        f.render_widget(placeholder, area);
        return;
    }

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.primary()))
                .title("WPM History")
                .title_style(subtitle_style(theme)),
        )
        .data(wpm_history)
        .style(primary_style(theme));

    f.render_widget(sparkline, area);
}

/// Render a sleek progress bar showing typing progress
#[allow(dead_code)]
pub fn render_progress_bar(f: &mut Frame, area: Rect, current: usize, total: usize, theme: &Theme) {
    let ratio = if total > 0 {
        current as f64 / total as f64
    } else {
        0.0
    };

    let percent = (ratio * 100.0) as u16;
    let label = format!("{}/{}", current, total);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.secondary()))
                .title("Progress")
                .title_style(subtitle_style(theme)),
        )
        .gauge_style(progress_bar_style(theme, ratio))
        .percent(percent)
        .label(Span::styled(label, Style::default().fg(theme.correct())));

    f.render_widget(gauge, area);
}

/// Render a compact inline progress indicator (no border)
pub fn render_inline_progress(current: usize, total: usize, theme: &Theme) -> Line<'static> {
    let ratio = if total > 0 { current as f64 / total as f64 } else { 0.0 };
    let filled = (ratio * 20.0) as usize;
    let empty = 20 - filled;

    let bar: String = format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(empty)
    );

    Line::from(vec![
        Span::styled(bar, progress_bar_style(theme, ratio)),
        Span::styled(format!(" {:.0}%", ratio * 100.0), Style::default().fg(theme.primary())),
    ])
}
