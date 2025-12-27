use crate::tile::Tile;

#[derive(Debug, Copy, Clone)]
pub enum Mentsu {
    Triplet(Tile),
    Quad(Tile),
    Sequence(Tile, Tile, Tile),
    Pair(Tile),
}

impl Mentsu {
    pub fn contains_terminal(self) -> bool {
        match self {
            Self::Triplet(t) | Self::Quad(t) | Self::Pair(t) => t.terminal(),
            Self::Sequence(t0, t1, t2) => t0.terminal() || t1.terminal() || t2.terminal(),
        }
    }

    pub fn entirely_terminal(self) -> bool {
        match self {
            Self::Triplet(t) | Self::Quad(t) | Self::Pair(t) => t.terminal(),
            Self::Sequence(_, _, _) => false,
        }
    }

    pub fn honor(self) -> bool {
        match self {
            Self::Triplet(t) | Self::Quad(t) | Self::Pair(t) => t.honor(),
            Self::Sequence(_, _, _) => false,
        }
    }

    pub const fn pair(self) -> bool {
        matches!(self, Self::Pair(_))
    }

    pub const fn quad(self) -> bool {
        matches!(self, Self::Quad(_))
    }

    pub const fn sequence(self) -> bool {
        matches!(self, Self::Sequence(_, _, _))
    }

    pub const fn triplet(self) -> bool {
        matches!(self, Self::Triplet(_) | Self::Quad(_))
    }
}
