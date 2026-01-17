pub mod finger_drill;
pub mod normal;
pub mod weak_letter;

use crate::finger_map::Finger;
use crate::stats::ProgressData;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PassageLength {
    Short,
    #[default]
    Medium,
    Long,
}

impl PassageLength {
    pub fn next(&self) -> Self {
        match self {
            PassageLength::Short => PassageLength::Medium,
            PassageLength::Medium => PassageLength::Long,
            PassageLength::Long => PassageLength::Short,
        }
    }

    pub fn word_count(&self) -> usize {
        match self {
            PassageLength::Short => 10,
            PassageLength::Medium => 25,
            PassageLength::Long => 50,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PassageLength::Short => "Short",
            PassageLength::Medium => "Medium",
            PassageLength::Long => "Long",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    WeakLetter,
    FingerDrill(Finger),
}

impl Mode {
    pub fn name(&self) -> String {
        match self {
            Mode::Normal => "Normal".to_string(),
            Mode::WeakLetter => "Weak Letters".to_string(),
            Mode::FingerDrill(finger) => format!("{} Drill", finger.name()),
        }
    }

    pub fn next(&self) -> Mode {
        match self {
            Mode::Normal => Mode::WeakLetter,
            Mode::WeakLetter => Mode::FingerDrill(Finger::LeftPinky),
            Mode::FingerDrill(finger) => {
                let fingers = Finger::all();
                let current_idx = fingers.iter().position(|f| f == finger).unwrap_or(0);
                if current_idx + 1 < fingers.len() {
                    Mode::FingerDrill(fingers[current_idx + 1])
                } else {
                    Mode::Normal
                }
            }
        }
    }

    pub fn previous(&self) -> Mode {
        match self {
            Mode::Normal => {
                let fingers = Finger::all();
                Mode::FingerDrill(*fingers.last().unwrap())
            }
            Mode::WeakLetter => Mode::Normal,
            Mode::FingerDrill(finger) => {
                let fingers = Finger::all();
                let current_idx = fingers.iter().position(|f| f == finger).unwrap_or(0);
                if current_idx > 0 {
                    Mode::FingerDrill(fingers[current_idx - 1])
                } else {
                    Mode::WeakLetter
                }
            }
        }
    }

    pub fn generate_text(&self, progress: &ProgressData, word_count: usize) -> String {
        match self {
            Mode::Normal => normal::generate_text(word_count),
            Mode::WeakLetter => weak_letter::generate_text(progress, word_count),
            Mode::FingerDrill(finger) => finger_drill::generate_text(*finger, word_count),
        }
    }
}
