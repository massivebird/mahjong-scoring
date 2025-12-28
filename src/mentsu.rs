use crate::tile::Tile;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone)]
pub struct Mentsu {
    pub kind: Kind,
    pub open: bool,
    pub win_wait: Option<WinWait>,
}

#[derive(Debug, Copy, Clone)]
pub enum WinWait {
    Ryanmen,
    Kanchan,
    Penchan,
    Tanki,
    Shanpon,
}

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Triplet(Tile),
    Quad(Tile),
    Sequence(Tile, Tile, Tile),
    Pair(Tile),
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

    pub fn entirely_terminal(self) -> bool {
        match self.kind {
            Kind::Triplet(t) | Kind::Quad(t) | Kind::Pair(t) => t.terminal(),
            Kind::Sequence(_, _, _) => false,
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

    dbg!(&counts);

    rec_build(&counts, 0, &[])
}

/// Recursively computes possible interpretations of a hand.
fn rec_build(counts: &BTreeMap<Tile, u32>, i: usize, mentsu_rn: &[Mentsu]) -> Vec<Vec<Mentsu>> {
    dbg!(mentsu_rn);
    let mut ans: Vec<Vec<Mentsu>> = vec![];

    // Check if we've exhausted all tiles.
    let Some((&this, &this_count)) = counts.iter().nth(i) else {
        return vec![mentsu_rn.to_vec()];
    };
    dbg!((this, this_count));

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
