use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::theme::{subtitle_style, Theme};

pub fn render_confirmation_dialog(f: &mut Frame, title: &str, message: &str, theme: &Theme) {
    // Create a centered popup
    let area = centered_rect(50, 30, f.area());

    // Clear the area behind the dialog
    f.render_widget(Clear, area);

    // Create the dialog block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.error()).add_modifier(Modifier::BOLD))
        .title(title)
        .title_style(Style::default().fg(theme.warning()).add_modifier(Modifier::BOLD));

    // Split the dialog area into message and controls sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(area);

    // Create message paragraph
    let message_widget = Paragraph::new(message)
        .style(subtitle_style(theme))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: false });

    // Create controls text
    let controls = vec![
        Span::styled("[Y] ", Style::default().fg(theme.correct()).add_modifier(Modifier::BOLD)),
        Span::raw("Yes  "),
        Span::styled("[N] ", Style::default().fg(theme.error()).add_modifier(Modifier::BOLD)),
        Span::raw("No  "),
        Span::styled("[Esc] ", Style::default().fg(theme.subtitle())),
        Span::raw("Cancel"),
    ];

    let controls_widget = Paragraph::new(Line::from(controls))
        .alignment(Alignment::Center);

    // Render the dialog
    f.render_widget(block, area);
    f.render_widget(message_widget, chunks[0]);
    f.render_widget(controls_widget, chunks[1]);
}

/// Helper function to create a centered rect using up certain percentage of the available rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
