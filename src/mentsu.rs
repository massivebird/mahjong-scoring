use crate::{suit::Suit, tile::Tile};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Mentsu {
    pub kind: Kind,
    pub open: bool,
    pub win_wait: Option<WinWait>,
}

impl std::fmt::Display for Mentsu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();

        if self.open {
            buf.push_str("Op");
        }

        buf.push_str(&self.kind.to_string());

        if let Some(wait) = self.win_wait {
            buf.push_str(&wait.to_string());
        }

        write!(f, "{buf}")
    }
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    Triplet(Tile),
    Quad(Tile),
    Sequence(Tile, Tile, Tile),
    Pair(Tile),
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Triplet(tile) => format!("Tr({tile})"),
            Self::Quad(tile) => format!("Qd({tile})"),
            Self::Sequence(t0, t1, t2) => format!("Sq({t0},{t1},{t2})"),
            Self::Pair(tile) => format!("Pr({tile})"),
        };

        write!(f, "{s}")
    }
}

impl Mentsu {
    pub const fn new(kind: Kind) -> Self {
        Self {
            kind,
            open: false,
            win_wait: None,
        }
    }

    pub fn contains(self, t: Tile) -> bool {
        match self.kind {
            Kind::Triplet(h) | Kind::Quad(h) | Kind::Pair(h) => t == h,
            Kind::Sequence(h0, h1, h2) => t == h0 || t == h1 || t == h2,
        }
    }

    pub const fn set_win_wait(&mut self, val: Option<WinWait>) {
        self.win_wait = val;
    }

    pub const fn set_open(&mut self, val: bool) {
        self.open = val;
    }

    pub fn contains_terminal(self) -> bool {
        match self.kind {
            Kind::Triplet(t) | Kind::Quad(t) | Kind::Pair(t) => t.terminal(),
            Kind::Sequence(t0, t1, t2) => t0.terminal() || t1.terminal() || t2.terminal(),
        }
    }

    pub const fn closed(self) -> bool {
        !self.open
    }

    pub fn entirely_terminal(self) -> bool {
        match self.kind {
            Kind::Triplet(t) | Kind::Quad(t) | Kind::Pair(t) => t.terminal(),
            Kind::Sequence(_, _, _) => false,
        }
    }

    pub const fn suit(self) -> Suit {
        match self.kind {
            Kind::Triplet(t) | Kind::Quad(t) | Kind::Pair(t) | Kind::Sequence(t, _, _) => t.suit,
        }
    }

    pub fn honor(self) -> bool {
        match self.kind {
            Kind::Triplet(t) | Kind::Quad(t) | Kind::Pair(t) => t.honor(),
            Kind::Sequence(_, _, _) => false,
        }
    }

    pub const fn pair(self) -> bool {
        matches!(self.kind, Kind::Pair(_))
    }

    pub const fn quad(self) -> bool {
        matches!(self.kind, Kind::Quad(_))
    }

    pub const fn sequence(self) -> bool {
        matches!(self.kind, Kind::Sequence(_, _, _))
    }

    pub const fn triplet(self) -> bool {
        matches!(self.kind, Kind::Triplet(_) | Kind::Quad(_))
    }
}
