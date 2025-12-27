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

pub fn build_mentsu(as_tiles: &[Tile]) -> Vec<Vec<Mentsu>> {
    rec_build(as_tiles, 0, &[])
}

/// Recursively computes possible interpretations of a hand.
fn rec_build(as_tiles: &[Tile], i: usize, mentsu_rn: &[Mentsu]) -> Vec<Vec<Mentsu>> {
    let mut ans: Vec<Vec<Mentsu>> = vec![];

    // Exhausted all tiles.
    if i >= as_tiles.len() {
        return vec![mentsu_rn.to_vec()];
    }

    let this = as_tiles[i];

    // Pair
    if i <= as_tiles.len() - 2 && this == as_tiles[i + 1] {
        for m in rec_build(as_tiles, i + 2, &with(mentsu_rn, Mentsu::Pair(this))) {
            ans.push(m);
        }
    }

    // Triplet
    if i <= as_tiles.len() - 3 && this == as_tiles[i + 1] && this == as_tiles[i + 2] {
        for m in rec_build(as_tiles, i + 3, &with(mentsu_rn, Mentsu::Triplet(this))) {
            ans.push(m);
        }

        // Sure let's check quads in here too
        if i <= as_tiles.len() - 4 && this == as_tiles[i + 3] {
            for m in rec_build(as_tiles, i + 4, &with(mentsu_rn, Mentsu::Quad(this))) {
                ans.push(m);
            }
        }
    }

    // Sequence
    if i <= as_tiles.len() - 3
        && this.can_sequence(as_tiles[i + 1])
        && this.can_sequence(as_tiles[i + 2])
    {
        for m in rec_build(
            as_tiles,
            i + 3,
            &with(
                mentsu_rn,
                Mentsu::Sequence(this, as_tiles[i + 1], as_tiles[i + 2]),
            ),
        ) {
            ans.push(m);
        }
    }

    ans
}

fn with(vec: &[Mentsu], val: Mentsu) -> Vec<Mentsu> {
    [vec, &[val]].concat()
}
