use ratatui::style::{Color, Modifier, Style};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    Synthwave,
    Dracula,
    OneDark,
    Monokai,
    Nord,
    #[default]
    Gruvbox,
}

impl Theme {
    pub fn next(&self) -> Theme {
        match self {
            Theme::Synthwave => Theme::Dracula,
            Theme::Dracula => Theme::OneDark,
            Theme::OneDark => Theme::Monokai,
            Theme::Monokai => Theme::Nord,
            Theme::Nord => Theme::Gruvbox,
            Theme::Gruvbox => Theme::Synthwave,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Theme::Synthwave => "Synthwave",
            Theme::Dracula => "Dracula",
            Theme::OneDark => "One Dark",
            Theme::Monokai => "Monokai",
            Theme::Nord => "Nord",
            Theme::Gruvbox => "Gruvbox",
        }
    }

    fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".ratatap");
        path.push("theme.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::get_path();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(theme) = serde_json::from_str(&data) {
                    return theme;
                }
            }
        }
        Theme::default()
    }

    pub fn save(&self) {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_string(self) {
            let _ = fs::write(path, data);
        }
    }

    /// Primary accent color (borders, highlights)
    pub fn primary(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(57, 255, 250),   // Neon cyan
            Theme::Dracula => Color::Rgb(189, 147, 249),    // Purple
            Theme::OneDark => Color::Rgb(97, 175, 239),     // Blue
            Theme::Monokai => Color::Rgb(102, 217, 239),    // Cyan
            Theme::Nord => Color::Rgb(136, 192, 208),       // Frost blue
            Theme::Gruvbox => Color::Rgb(251, 189, 46),     // Yellow
        }
    }

    /// Secondary accent color
    pub fn secondary(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(191, 64, 255),   // Neon purple
            Theme::Dracula => Color::Rgb(255, 121, 198),    // Pink
            Theme::OneDark => Color::Rgb(198, 120, 221),    // Purple
            Theme::Monokai => Color::Rgb(174, 129, 255),    // Purple
            Theme::Nord => Color::Rgb(180, 142, 173),       // Purple
            Theme::Gruvbox => Color::Rgb(211, 134, 155),    // Purple
        }
    }

    /// Correct text color (green-ish)
    pub fn correct(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(57, 255, 20),    // Neon green
            Theme::Dracula => Color::Rgb(80, 250, 123),     // Green
            Theme::OneDark => Color::Rgb(152, 195, 121),    // Green
            Theme::Monokai => Color::Rgb(166, 226, 46),     // Green
            Theme::Nord => Color::Rgb(163, 190, 140),       // Green
            Theme::Gruvbox => Color::Rgb(184, 187, 38),     // Green
        }
    }

    /// Error/incorrect text color (red/pink-ish)
    pub fn error(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(255, 105, 180),  // Hot pink
            Theme::Dracula => Color::Rgb(255, 85, 85),      // Red
            Theme::OneDark => Color::Rgb(224, 108, 117),    // Red
            Theme::Monokai => Color::Rgb(249, 38, 114),     // Pink
            Theme::Nord => Color::Rgb(191, 97, 106),        // Red
            Theme::Gruvbox => Color::Rgb(251, 73, 52),      // Red
        }
    }

    /// Current character foreground
    pub fn current_fg(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(20, 20, 30),     // Dark
            Theme::Dracula => Color::Rgb(40, 42, 54),       // Background
            Theme::OneDark => Color::Rgb(40, 44, 52),       // Background
            Theme::Monokai => Color::Rgb(39, 40, 34),       // Background
            Theme::Nord => Color::Rgb(46, 52, 64),          // Background
            Theme::Gruvbox => Color::Rgb(40, 40, 40),       // Background
        }
    }

    /// Current character background (high visibility)
    pub fn current_bg(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(255, 255, 100),  // Bright yellow
            Theme::Dracula => Color::Rgb(241, 250, 140),    // Yellow
            Theme::OneDark => Color::Rgb(229, 192, 123),    // Yellow
            Theme::Monokai => Color::Rgb(230, 219, 116),    // Yellow
            Theme::Nord => Color::Rgb(235, 203, 139),       // Yellow
            Theme::Gruvbox => Color::Rgb(250, 189, 47),     // Yellow
        }
    }

    /// Pending/untyped text color (dimmed)
    pub fn pending(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(100, 100, 140),
            Theme::Dracula => Color::Rgb(98, 114, 164),     // Comment
            Theme::OneDark => Color::Rgb(92, 99, 112),      // Comment
            Theme::Monokai => Color::Rgb(117, 113, 94),     // Comment
            Theme::Nord => Color::Rgb(76, 86, 106),         // Comment
            Theme::Gruvbox => Color::Rgb(146, 131, 116),    // Gray
        }
    }

    /// Subtitle/label color
    pub fn subtitle(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(180, 180, 220),
            Theme::Dracula => Color::Rgb(139, 148, 158),
            Theme::OneDark => Color::Rgb(171, 178, 191),
            Theme::Monokai => Color::Rgb(166, 172, 163),
            Theme::Nord => Color::Rgb(216, 222, 233),
            Theme::Gruvbox => Color::Rgb(189, 174, 147),
        }
    }

    /// Warning color (orange/yellow)
    pub fn warning(&self) -> Color {
        match self {
            Theme::Synthwave => Color::Rgb(255, 176, 0),
            Theme::Dracula => Color::Rgb(255, 184, 108),    // Orange
            Theme::OneDark => Color::Rgb(209, 154, 102),    // Orange
            Theme::Monokai => Color::Rgb(253, 151, 31),     // Orange
            Theme::Nord => Color::Rgb(208, 135, 112),       // Orange
            Theme::Gruvbox => Color::Rgb(254, 128, 25),     // Orange
        }
    }
}

// Style helper functions that take a theme parameter

pub fn primary_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.primary())
        .add_modifier(Modifier::BOLD)
}

pub fn correct_char_style(theme: &Theme) -> Style {
    Style::default().fg(theme.correct())
}

pub fn incorrect_char_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.error())
        .add_modifier(Modifier::BOLD)
}

pub fn current_char_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.current_fg())
        .bg(theme.current_bg())
        .add_modifier(Modifier::BOLD)
}

pub fn pending_char_style(theme: &Theme) -> Style {
    Style::default().fg(theme.pending())
}

pub fn subtitle_style(theme: &Theme) -> Style {
    Style::default().fg(theme.subtitle())
}

pub fn wpm_color(theme: &Theme, wpm: f64) -> Color {
    if wpm >= 60.0 {
        theme.correct()
    } else if wpm >= 40.0 {
        Color::Rgb(200, 255, 100)
    } else if wpm >= 20.0 {
        theme.warning()
    } else {
        theme.error()
    }
}

/// Color interpolation for gradient effects
pub fn interpolate_color(color1: Color, color2: Color, ratio: f64) -> Color {
    let ratio = ratio.clamp(0.0, 1.0);

    match (color1, color2) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * ratio) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * ratio) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * ratio) as u8;
            Color::Rgb(r, g, b)
        }
        _ => color1,
    }
}

/// Style for the currently highlighted key on the keyboard
pub fn current_key_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.current_fg())
        .bg(theme.current_bg())
        .add_modifier(Modifier::BOLD)
}

/// Get color for a key based on its error count
pub fn key_error_color(theme: &Theme, error_count: usize) -> Color {
    if error_count <= 3 {
        theme.correct()         // 0-3 errors: good
    } else if error_count <= 8 {
        interpolate_color(theme.correct(), theme.warning(), 0.5)
    } else if error_count <= 15 {
        theme.warning()         // 9-15: needs practice
    } else if error_count <= 25 {
        interpolate_color(theme.warning(), theme.error(), 0.5)
    } else {
        theme.error()           // 25+: major weakness
    }
}

/// Style for rendering a progress bar
pub fn progress_bar_style(theme: &Theme, ratio: f64) -> Style {
    let color = interpolate_color(theme.error(), theme.correct(), ratio);
    Style::default().fg(color)
}
