mod finger_map;
mod modes;
mod stats;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    collections::HashMap,
    error::Error,
    io,
    time::{Duration, Instant},
};

use chrono::Utc;
use finger_map::Finger;
use modes::{Mode, PassageLength};
use stats::{ProgressData, TestResult};
use ui::{
    charts::{render_inline_progress, render_wpm_sparkline},
    keyboard::render_keyboard,
    theme::{Theme, correct_char_style, incorrect_char_style, current_char_style, pending_char_style, subtitle_style, wpm_color, interpolate_color},
};
use ratatui::style::Modifier;

struct App {
    mode: Mode,
    target_text: String,
    typed_text: String,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    char_errors: HashMap<char, usize>,
    progress: ProgressData,
    current_wpm: f64,
    current_accuracy: f64,
    should_quit: bool,
    show_reset_confirmation: bool,
    theme: Theme,
    passage_length: PassageLength,
}

impl App {
    fn new() -> Self {
        let progress = ProgressData::load();
        let mode = Mode::Normal;
        let passage_length = PassageLength::default();
        let target_text = mode.generate_text(&progress, passage_length.word_count());

        App {
            mode,
            target_text,
            typed_text: String::new(),
            start_time: None,
            end_time: None,
            char_errors: HashMap::new(),
            progress,
            current_wpm: 0.0,
            current_accuracy: 0.0,
            should_quit: false,
            show_reset_confirmation: false,
            theme: Theme::load(),
            passage_length,
        }
    }

    fn reset_test(&mut self) {
        self.target_text = self.mode.generate_text(&self.progress, self.passage_length.word_count());
        self.typed_text = String::new();
        self.start_time = None;
        self.end_time = None;
        self.char_errors = HashMap::new();
        self.current_wpm = 0.0;
        self.current_accuracy = 0.0;
    }

    fn change_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
        self.reset_test();
    }

    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Handle confirmation dialog keys first
        if self.show_reset_confirmation {
            match key {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    // Clear history and reset
                    let _ = ProgressData::clear_history();
                    self.progress = ProgressData::load();
                    self.reset_test();
                    self.show_reset_confirmation = false;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    // Cancel confirmation
                    self.show_reset_confirmation = false;
                }
                _ => {}
            }
            return;
        }

        match key {
            KeyCode::Tab => {
                self.change_mode(self.mode.next());
            }
            KeyCode::BackTab => {
                self.change_mode(self.mode.previous());
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Enter => {
                if self.end_time.is_some() {
                    // Test already complete - start a new one
                    self.reset_test();
                } else if !self.typed_text.is_empty() {
                    // Mid-test - finish early
                    self.finish_test();
                }
            }
            KeyCode::Backspace => {
                if self.end_time.is_none() && !self.typed_text.is_empty() {
                    self.typed_text.pop();
                    self.calculate_stats();
                }
            }
            KeyCode::Char('r') if modifiers.contains(KeyModifiers::CONTROL) && self.typed_text.is_empty() => {
                // Ctrl+R to reset history (only when not actively typing)
                self.show_reset_confirmation = true;
            }
            KeyCode::Char('t') if modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+T to cycle themes
                self.theme = self.theme.next();
                self.theme.save();
            }
            KeyCode::Char('l') if modifiers.contains(KeyModifiers::CONTROL) => {
                // Ctrl+L to cycle passage length
                self.passage_length = self.passage_length.next();
                self.reset_test();
            }
            KeyCode::Char('1') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::LeftPinky));
            }
            KeyCode::Char('2') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::LeftRing));
            }
            KeyCode::Char('3') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::LeftMiddle));
            }
            KeyCode::Char('4') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::LeftIndex));
            }
            KeyCode::Char('6') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::RightIndex));
            }
            KeyCode::Char('7') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::RightMiddle));
            }
            KeyCode::Char('8') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::RightRing));
            }
            KeyCode::Char('9') if self.end_time.is_none() || self.typed_text.is_empty() => {
                self.change_mode(Mode::FingerDrill(Finger::RightPinky));
            }
            KeyCode::Char(c) => {
                if self.end_time.is_none() {
                    if self.start_time.is_none() {
                        self.start_time = Some(Instant::now());
                    }

                    if self.typed_text.len() < self.target_text.len() {
                        self.typed_text.push(c);

                        let target_char = self.target_text.chars().nth(self.typed_text.len() - 1);
                        if target_char != Some(c) {
                            if let Some(tc) = target_char {
                                *self.char_errors.entry(tc).or_insert(0) += 1;
                            }
                        }

                        if self.typed_text.len() == self.target_text.len() {
                            self.finish_test();
                        }

                        self.calculate_stats();
                    }
                }
            }
            _ => {}
        }
    }

    fn calculate_stats(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                // Calculate accuracy by comparing typed chars to target chars directly
                let correct_count = self.typed_text
                    .chars()
                    .zip(self.target_text.chars())
                    .filter(|(typed, target)| typed == target)
                    .count();

                // WPM based on correct characters only (net WPM)
                let correct_chars = correct_count as f64;
                let words_typed = correct_chars / 5.0;
                let minutes = elapsed / 60.0;
                self.current_wpm = words_typed / minutes;

                if !self.typed_text.is_empty() {
                    self.current_accuracy = (correct_count as f64 / self.typed_text.len() as f64) * 100.0;
                } else {
                    self.current_accuracy = 100.0;
                }
            }
        }
    }

    fn finish_test(&mut self) {
        self.end_time = Some(Instant::now());

        if let Some(start) = self.start_time {
            let duration = start.elapsed();

            let result = TestResult {
                wpm: self.current_wpm,
                accuracy: self.current_accuracy,
                timestamp: Utc::now(),
                duration_secs: duration.as_secs(),
                char_errors: self.char_errors.clone(),
            };

            self.progress.results.push(result);
            let _ = self.progress.save();
        }
    }

    fn get_current_char(&self) -> Option<char> {
        if self.typed_text.len() < self.target_text.len() {
            self.target_text.chars().nth(self.typed_text.len())
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code, key.modifiers);
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let terminal_height = f.area().height;

    let (keyboard_height, stats_height) = if terminal_height < 25 {
        (0, 3)  // Hide keyboard in very small terminals
    } else {
        (8, 5)  // Show keyboard with full stats - increased for better spacing
    };

    let mut constraints = vec![
        Constraint::Length(9),     // Title with ASCII art logo
        Constraint::Min(6),        // Text to type - increased for readability
    ];

    if keyboard_height > 0 {
        constraints.push(Constraint::Length(keyboard_height));
    }

    constraints.push(Constraint::Length(stats_height)); // Stats
    constraints.push(Constraint::Length(4));            // Bottom info - increased for better spacing

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.area());

    // ASCII art logo with gradient colors
    // All lines are the same width to maintain slant alignment
    let logo_lines = [
        r"    ____        __        __         ____   /\_/\ ~~, ",
        r"   / __ \____ _/ /_____ _/ /_____ __/ __ \ ( o.o )  / ",
        r"  / /_/ / __ `/ __/ __ `/ __/ __ `/ /_/ /   > ^ <  /  ",
        r" / _, _/ /_/ / /_/ /_/ / /_/ /_/ / ____/   /|   |\/   ",
        r"/_/ |_|\__,_/\__/\__,_/\__/\__,_/_/       (_|   |_)   ",
    ];

    let mut title_lines: Vec<Line> = Vec::new();
    let logo_width = logo_lines[0].len();

    // Add each logo line with gradient
    for (line_idx, logo_line) in logo_lines.iter().enumerate() {
        let mut spans = Vec::new();
        for (i, ch) in logo_line.chars().enumerate() {
            // Gradient based on horizontal position and line
            let h_ratio = i as f64 / logo_width.max(1) as f64;
            let v_ratio = line_idx as f64 / logo_lines.len() as f64;
            let ratio = (h_ratio + v_ratio) / 2.0;
            let color = interpolate_color(app.theme.primary(), app.theme.secondary(), ratio);
            spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        }
        title_lines.push(Line::from(spans));
    }

    // Add mode/theme/length indicator line (centered within logo width)
    let indicator = format!("[{}] {} ({})", app.theme.name(), app.mode.name(), app.passage_length.name());
    let padded_indicator = format!("{:^width$}", indicator, width = logo_width);
    title_lines.push(Line::from(Span::styled(
        padded_indicator,
        subtitle_style(&app.theme),
    )));

    let title = Paragraph::new(title_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.primary()))
        );
    f.render_widget(title, main_chunks[0]);

    let mut text_spans = Vec::new();
    let typed_chars: Vec<char> = app.typed_text.chars().collect();
    let target_chars: Vec<char> = app.target_text.chars().collect();

    for (i, &target_char) in target_chars.iter().enumerate() {
        let style = if i < typed_chars.len() {
            if typed_chars[i] == target_char {
                correct_char_style(&app.theme)
            } else {
                incorrect_char_style(&app.theme)
            }
        } else if i == typed_chars.len() {
            current_char_style(&app.theme)
        } else {
            pending_char_style(&app.theme)
        };
        text_spans.push(Span::styled(target_char.to_string(), style));
    }

    let text_widget = Paragraph::new(Line::from(text_spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.secondary()))
                .title("Text to Type")
                .title_style(subtitle_style(&app.theme)),
        )
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left);
    f.render_widget(text_widget, main_chunks[1]);

    let mut chunk_idx = 2;

    if keyboard_height > 0 {
        // Merge historical errors with current session errors for real-time feedback
        let mut char_stats = app.progress.get_char_error_analysis();
        for (&ch, &count) in &app.char_errors {
            let stats = char_stats.entry(ch).or_insert(stats::CharStats {
                total_errors: 0,
                total_appearances: 0,
            });
            stats.total_errors += count;
        }
        render_keyboard(
            f,
            main_chunks[chunk_idx],
            app.get_current_char(),
            &char_stats,
            &app.theme,
        );
        chunk_idx += 1;
    }

    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[chunk_idx]);

    let total_errors: usize = app.char_errors.values().sum();

    // Build stats content with styled spans
    let acc_color = if app.current_accuracy >= 95.0 {
        app.theme.correct()
    } else if app.current_accuracy >= 90.0 {
        app.theme.secondary()
    } else {
        app.theme.error()
    };

    let stats_lines = if app.end_time.is_some() {
        vec![
            Line::from(Span::styled("✓ Test Complete!", Style::default().fg(app.theme.correct()).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(vec![
                Span::styled("WPM: ", subtitle_style(&app.theme)),
                Span::styled(format!("{:.1}", app.current_wpm), Style::default().fg(wpm_color(&app.theme, app.current_wpm)).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled("Acc: ", subtitle_style(&app.theme)),
                Span::styled(format!("{:.1}%", app.current_accuracy), Style::default().fg(app.theme.correct())),
            ]),
            Line::from(""),
            Line::from(Span::styled("Press Enter for new test", subtitle_style(&app.theme))),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("WPM: ", subtitle_style(&app.theme)),
                Span::styled(format!("{:.1}", app.current_wpm), Style::default().fg(wpm_color(&app.theme, app.current_wpm)).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled("Acc: ", subtitle_style(&app.theme)),
                Span::styled(format!("{:.1}%", app.current_accuracy), Style::default().fg(acc_color)),
            ]),
            Line::from(vec![
                Span::styled("Errors: ", subtitle_style(&app.theme)),
                Span::styled(format!("{}", total_errors), Style::default().fg(if total_errors == 0 { app.theme.correct() } else { app.theme.error() })),
            ]),
            render_inline_progress(app.typed_text.len(), app.target_text.len(), &app.theme),
        ]
    };

    let stats_widget = Paragraph::new(stats_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.primary()))
                .title("Stats")
                .title_style(subtitle_style(&app.theme)),
        )
        .alignment(Alignment::Left);
    f.render_widget(stats_widget, stats_chunks[0]);

    let wpm_history = app.progress.get_wpm_history(20);
    render_wpm_sparkline(f, stats_chunks[1], &wpm_history, &app.theme);

    chunk_idx += 1;

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[chunk_idx]);

    // History panel with styled content
    let history_lines = vec![
        Line::from(vec![
            Span::styled("Tests: ", subtitle_style(&app.theme)),
            Span::styled(format!("{}", app.progress.results.len()), Style::default().fg(app.theme.primary()).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("Avg WPM: ", subtitle_style(&app.theme)),
            Span::styled(format!("{:.1}", app.progress.average_wpm()), Style::default().fg(wpm_color(&app.theme, app.progress.average_wpm()))),
            Span::raw("  "),
            Span::styled("Avg Acc: ", subtitle_style(&app.theme)),
            Span::styled(format!("{:.1}%", app.progress.average_accuracy()), Style::default().fg(app.theme.correct())),
        ]),
    ];

    let history_widget = Paragraph::new(history_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.secondary()))
                .title("History")
                .title_style(subtitle_style(&app.theme)),
        )
        .alignment(Alignment::Center);
    f.render_widget(history_widget, bottom_chunks[0]);

    // Controls panel with styled shortcuts
    let controls = Line::from(vec![
        Span::styled("Tab", Style::default().fg(app.theme.primary()).add_modifier(Modifier::BOLD)),
        Span::styled(":Mode ", subtitle_style(&app.theme)),
        Span::styled("1-4,6-9", Style::default().fg(app.theme.primary()).add_modifier(Modifier::BOLD)),
        Span::styled(":Finger ", subtitle_style(&app.theme)),
        Span::styled("^T", Style::default().fg(app.theme.warning()).add_modifier(Modifier::BOLD)),
        Span::styled(":Theme ", subtitle_style(&app.theme)),
        Span::styled("^L", Style::default().fg(app.theme.warning()).add_modifier(Modifier::BOLD)),
        Span::styled(":Length ", subtitle_style(&app.theme)),
        Span::styled("Enter", Style::default().fg(app.theme.correct()).add_modifier(Modifier::BOLD)),
        Span::styled(":Retry ", subtitle_style(&app.theme)),
        Span::styled("^R", Style::default().fg(app.theme.error()).add_modifier(Modifier::BOLD)),
        Span::styled(":Reset ", subtitle_style(&app.theme)),
        Span::styled("Esc", Style::default().fg(app.theme.secondary()).add_modifier(Modifier::BOLD)),
        Span::styled(":Quit", subtitle_style(&app.theme)),
    ]);

    let controls_widget = Paragraph::new(controls)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.secondary()))
                .title("Controls")
                .title_style(subtitle_style(&app.theme)),
        )
        .alignment(Alignment::Center);
    f.render_widget(controls_widget, bottom_chunks[1]);

    // Render confirmation dialog on top if active
    if app.show_reset_confirmation {
        ui::dialogs::render_confirmation_dialog(
            f,
            " ⚠ Reset All History? ",
            "This will delete all test results\nand statistics.\n\nThis action cannot be undone.",
            &app.theme,
        );
    }
}
