use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Suit {
    Man,
    Pin,
    Sou,
    Honor,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Man => "m",
                Self::Pin => "p",
                Suit::Sou => "s",
                Suit::Honor => "z",
            }
        )
    }
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
