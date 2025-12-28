#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WinWait {
    Ryanmen,
    Kanchan,
    Penchan,
    Tanki,
    Shanpon,
}

impl std::fmt::Display for WinWait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ryanmen => "RYN",
                Self::Kanchan => "KAN",
                Self::Penchan => "PEN",
                Self::Tanki => "TAN",
                Self::Shanpon => "SHA",
            }
        )
    }
}

