use crate::tile::Tile;

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
