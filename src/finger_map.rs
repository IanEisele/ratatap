use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

impl Finger {
    pub fn name(&self) -> &str {
        match self {
            Finger::LeftPinky => "Left Pinky",
            Finger::LeftRing => "Left Ring",
            Finger::LeftMiddle => "Left Middle",
            Finger::LeftIndex => "Left Index",
            Finger::RightIndex => "Right Index",
            Finger::RightMiddle => "Right Middle",
            Finger::RightRing => "Right Ring",
            Finger::RightPinky => "Right Pinky",
        }
    }

    pub fn all() -> Vec<Finger> {
        vec![
            Finger::LeftPinky,
            Finger::LeftRing,
            Finger::LeftMiddle,
            Finger::LeftIndex,
            Finger::RightIndex,
            Finger::RightMiddle,
            Finger::RightRing,
            Finger::RightPinky,
        ]
    }
}

impl fmt::Display for Finger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn get_keys_for_finger(finger: Finger) -> Vec<char> {
    match finger {
        Finger::LeftPinky => vec!['`', '1', 'q', 'a', 'z'],
        Finger::LeftRing => vec!['2', 'w', 's', 'x'],
        Finger::LeftMiddle => vec!['3', 'e', 'd', 'c'],
        Finger::LeftIndex => vec!['4', '5', 'r', 't', 'f', 'g', 'v', 'b'],
        Finger::RightIndex => vec!['6', '7', 'y', 'u', 'h', 'j', 'n', 'm'],
        Finger::RightMiddle => vec!['8', 'i', 'k', ','],
        Finger::RightRing => vec!['9', 'o', 'l', '.'],
        Finger::RightPinky => vec!['0', '-', '=', 'p', '[', ']', ';', '\'', '/', '\\'],
    }
}
