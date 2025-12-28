use crate::mentsu::kind::Kind;
use crate::tile::Tile;

use super::Mentsu;

pub struct Iter {
    tiles: Vec<Tile>,
    next_idx: usize,
}

impl Iter {
    pub fn new(m: &Mentsu) -> Self {
        let tiles = match m.kind {
            Kind::Triplet(tile) | Kind::Quad(tile) | Kind::Pair(tile) => vec![tile; 3],
            Kind::Sequence(t0, t1, t2) => vec![t0, t1, t2],
        };

        Self { tiles, next_idx: 0 }
    }
}

impl Iterator for Iter {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tiles.get(self.next_idx) {
            Some(&t) => {
                self.next_idx += 1;
                Some(t)
            }
            _ => None,
        }
    }
}
