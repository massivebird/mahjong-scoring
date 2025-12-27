use std::collections::HashMap;

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
    let mut counts: HashMap<Tile, u32> = HashMap::new();

    for t in as_tiles {
        counts.entry(*t).and_modify(|v| *v += 1).or_insert(1);
    }

    rec_build(&counts, 0, &[])
}

/// Recursively computes possible interpretations of a hand.
fn rec_build(counts: &HashMap<Tile, u32>, i: usize, mentsu_rn: &[Mentsu]) -> Vec<Vec<Mentsu>> {
    dbg!(mentsu_rn);
    let mut ans: Vec<Vec<Mentsu>> = vec![];

    let Some((&this, &this_count)) = counts.iter().nth(i) else {
        return vec![mentsu_rn.to_vec()];
    };
    dbg!(this);

    if this_count == 0 {
        return rec_build(counts, i + 1, mentsu_rn);
    }

    // Pair
    if this_count >= 2 {
        for m in rec_build(
            &decrement(counts, &[this; 2]),
            i,
            &with(mentsu_rn, Mentsu::Pair(this)),
        ) {
            ans.push(m);
        }
    }

    // Triplet
    if this_count >= 3 {
        for m in rec_build(
            &decrement(counts, &[this; 3]),
            i,
            &with(mentsu_rn, Mentsu::Triplet(this)),
        ) {
            ans.push(m);
        }
    }

    // Quad
    if this_count >= 4 {
        for m in rec_build(
            &decrement(counts, &[this; 4]),
            i,
            &with(mentsu_rn, Mentsu::Quad(this)),
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
                Mentsu::Sequence(this, this.add(1).unwrap(), this.add(2).unwrap()),
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

fn decrement(counts: &HashMap<Tile, u32>, tiles: &[Tile]) -> HashMap<Tile, u32> {
    let mut res = counts.clone();

    for t in tiles {
        res.entry(*t).and_modify(|v| *v -= 1);
    }

    res
}
