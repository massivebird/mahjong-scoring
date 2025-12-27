#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Man,
    Pin,
    Sou,
    Honor,
}

impl From<char> for Suit {
    fn from(value: char) -> Self {
        match value {
            'm' => Self::Man,
            's' => Self::Sou,
            'p' => Self::Pin,
            'z' => Self::Honor,
            _ => unreachable!(),
        }
    }
}
