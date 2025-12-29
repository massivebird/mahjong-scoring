use crate::{
    mentsu::{Mentsu, kind::Kind},
    parser::win_wait::WinWait,
    tile::Tile,
};
use std::collections::{BTreeMap, HashSet};

use super::WinMethod;

pub fn build(as_tiles: &[Tile], win_tile: Tile, win_method: WinMethod) -> Vec<Vec<Mentsu>> {
    let mut counts: BTreeMap<Tile, u32> = BTreeMap::new();

    for t in as_tiles {
        counts.entry(*t).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut basic = rec_build(&counts, 0, &[]);

    // Keep only winning hands
    basic.retain(|v| {
        v.iter()
            .filter(|m| {
                matches!(
                    m.kind,
                    Kind::Triplet(_) | Kind::Quad(_) | Kind::Sequence(_, _, _)
                )
            })
            .count()
            == 4
            && v.iter().filter(|m| matches!(m.kind, Kind::Pair(_))).count() == 1
    });

    basic_to_open(&basic, win_tile, win_method)
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

fn basic_to_open(i13s: &[Vec<Mentsu>], win_tile: Tile, win_method: WinMethod) -> Vec<Vec<Mentsu>> {
    let mut ans: HashSet<Vec<Mentsu>> = HashSet::new();

    let mut i13s = i13s.to_owned();

    for hand in &mut i13s {
        for (i, m) in hand.iter().enumerate() {
            if m.contains(win_tile) {
                // Push a copy of this hand with this mentsu as open.
                let mut h = hand.clone();

                if win_method == WinMethod::Ron {
                    h[i].set_open(true);
                }

                let mut set_wait = |wait| h[i].set_win_wait(Some(wait));

                match m.kind {
                    // Tanki
                    Kind::Pair(_) => set_wait(WinWait::Tanki),

                    // Shanpon
                    Kind::Triplet(_) => set_wait(WinWait::Shanpon),

                    // Kanchan
                    Kind::Sequence(_, mid, _) if win_tile == mid => {
                        set_wait(WinWait::Kanchan);
                    }

                    // Penchan
                    Kind::Sequence(left, _, right)
                        if win_tile == left && right.terminal()
                            || win_tile == right && left.terminal() =>
                    {
                        set_wait(WinWait::Penchan);
                    }

                    // Ryanmen
                    Kind::Sequence(_, _, _) => set_wait(WinWait::Ryanmen),

                    // Where do quads fit into this? idk
                    Kind::Quad(_) => unimplemented!(),
                }

                // By sorting before inserting, hands are effectively
                // hashed/compared as unordered collections.
                h.sort();
                ans.insert(h);
            }
        }
    }

    ans.into_iter().collect()
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
