# Ratatap

A terminal-based touch typing practice application built with Rust and Ratatui.

```
                 ██████╗  █████╗ ████████╗ █████╗ ████████╗ █████╗ ██████╗
                 ██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗╚══██╔══╝██╔══██╗██╔══██╗
                 ██████╔╝███████║   ██║   ███████║   ██║   ███████║██████╔╝
                 ██╔══██╗██╔══██║   ██║   ██╔══██║   ██║   ██╔══██║██╔═══╝
                 ██║  ██║██║  ██║   ██║   ██║  ██║   ██║   ██║  ██║██║
                 ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝
```

## Features

- **Multiple Practice Modes**
  - **Normal Mode** - Type randomly selected common English words
  - **Weak Letter Mode** - Practice words containing letters you frequently mistype
  - **Finger Drill Mode** - Targeted practice for specific fingers (all 8 fingers supported)

- **Real-time Feedback**
  - Live WPM (Words Per Minute) tracking
  - Accuracy percentage with color-coded feedback
  - Visual keyboard showing error frequency per key
  - WPM sparkline chart of recent attempts

- **Customization**
  - 6 built-in themes: Synthwave, Dracula, OneDark, Monokai, Nord, Gruvbox
  - 3 passage lengths: Short (10 words), Medium (25 words), Long (50 words)
  - Persistent settings saved to `~/.ratatap/`

- **Progress Tracking**
  - All test results saved with timestamps
  - Historical error analysis to identify weak points
  - Average WPM and accuracy across all tests

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/ratatap.git
cd ratatap

# Build and run
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| `Tab` | Next mode |
| `Shift+Tab` | Previous mode |
| `1-4, 6-9` | Switch to specific finger drill |
| `Ctrl+T` | Cycle themes |
| `Ctrl+L` | Cycle passage length |
| `Enter` | Finish/start new test |
| `Backspace` | Delete last character |
| `Ctrl+R` | Reset all history |
| `Esc` | Quit |

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Ratatui](https://ratatui.rs/) - Terminal UI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal handling

## License

MIT
