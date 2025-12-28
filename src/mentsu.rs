use crate::suit::Suit;
use crate::tile::Tile;
use std::collections::BTreeMap;

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
        write!(f, "{}", match self {
            Self::Ryanmen => "RMN",
            Self::Kanchan => "KCN",
            Self::Penchan => "PCN",
            Self::Tanki => "TNK",
            Self::Shanpon => "SHP",
        })
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

pub fn build_mentsu(as_tiles: &[Tile]) -> Vec<Vec<Mentsu>> {
    let mut counts: BTreeMap<Tile, u32> = BTreeMap::new();

    for t in as_tiles {
        counts.entry(*t).and_modify(|v| *v += 1).or_insert(1);
    }

    rec_build(&counts, 0, &[])
}

/// Recursively computes possible interpretations of a hand.
fn rec_build(counts: &BTreeMap<Tile, u32>, i: usize, mentsu_rn: &[Mentsu]) -> Vec<Vec<Mentsu>> {
    let mut ans: Vec<Vec<Mentsu>> = vec![];

    // Check if we've exhausted all tiles.
    let Some((&this, &this_count)) = counts.iter().nth(i) else {
        return vec![mentsu_rn.to_vec()];
    };

    // This tile has been exhausted. Try the next one.
    if this_count == 0 {
        return rec_build(counts, i + 1, mentsu_rn);
    }

    // Pair
    if this_count >= 2 {
        for m in rec_build(
            &decrement(counts, &[this; 2]),
            i,
            &with(mentsu_rn, Mentsu::new(Kind::Pair(this))),
        ) {
            ans.push(m);
        }
    }

    // Triplet
    if this_count >= 3 {
        for m in rec_build(
            &decrement(counts, &[this; 3]),
            i,
            &with(mentsu_rn, Mentsu::new(Kind::Triplet(this))),
        ) {
            ans.push(m);
        }
    }

    // Quad
    if this_count >= 4 {
        for m in rec_build(
            &decrement(counts, &[this; 4]),
            i,
            &with(mentsu_rn, Mentsu::new(Kind::Quad(this))),
        ) {
            ans.push(m);
        }
    }

    // Sequence
    if this
        .add(1)
        .is_some_and(|t| counts.get(&t).is_some_and(|v| *v >= 1))
        && this
            .add(2)
            .is_some_and(|t| counts.get(&t).is_some_and(|v| *v >= 1))
    {
        for m in rec_build(
            &decrement(counts, &[this, this.add(1).unwrap(), this.add(2).unwrap()]),
            i,
            &with(
                mentsu_rn,
                Mentsu::new(Kind::Sequence(
                    this,
                    this.add(1).unwrap(),
                    this.add(2).unwrap(),
                )),
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

fn decrement(counts: &BTreeMap<Tile, u32>, tiles: &[Tile]) -> BTreeMap<Tile, u32> {
    let mut res = counts.clone();

    for t in tiles {
        res.entry(*t).and_modify(|v| *v -= 1);
    }

    res
}
