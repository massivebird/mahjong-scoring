use self::kind::Kind;
use crate::parser::WinWait;
use crate::tile::{Suit, Tile};

mod iter;
pub mod kind;

#[derive(Debug, Copy, Clone, Eq, PartialOrd, Ord)]
pub struct Mentsu {
    pub kind: Kind,
    pub open: bool,
    pub win_wait: Option<WinWait>,
}

impl PartialEq for Mentsu {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl std::hash::Hash for Mentsu {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.open.hash(state);
        self.win_wait.hash(state);
    }
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

    /// Returns `true` if and only if both mentsu fulfill all 3 conditions:
    ///
    /// (1) Same type (e.g. triplet, pair)
    /// (2) Identical values
    /// (3) Different suits
    ///
    /// Used for scoring sanshoku doujun and sanshoku doukou.
    pub fn eq_diff_suits(self, other: Self) -> bool {
        if self.suit() == other.suit() {
            return false;
        }

        match (self.kind, other.kind) {
            (Kind::Sequence(a, _, _), Kind::Sequence(b, _, _))
            | (Kind::Triplet(a), Kind::Triplet(b))
            | (Kind::Quad(a), Kind::Quad(b))
            | (Kind::Pair(a), Kind::Pair(b)) => a.value == b.value,
            _ => false,
        }
    }

    pub fn iter(&self) -> self::iter::Iter {
        self::iter::Iter::new(self)
    }
}

pub fn get_tiles(ms: &[Mentsu]) -> Vec<Tile> {
    ms.iter()
        .flat_map(|m| m.iter().collect::<Vec<Tile>>())
        .collect()
}
