use crate::tile::Tile;

use super::OpenScore;

pub struct WeirdYaku {
    pub name: &'static str,
    pub desc: &'static str,
    pub han: u32,
    pub open_score: OpenScore,
    pub f: fn(&[Tile]) -> bool,
}

/// These are tougher to evaluate with regular mentsu parsing.
pub static WEIRD_YAKU: &[WeirdYaku] = &[
    WeirdYaku {
        name: "Chiitoi",
        desc: "Seven (7) pairs",
        han: 2,
        open_score: OpenScore::Illegal,
        f: |tiles| {
            for (i, t) in tiles.iter().enumerate() {
                if tiles
                    .iter()
                    .enumerate()
                    .filter(|(j, o)| *j != i && *o == t)
                    .count()
                    != 1
                {
                    return false;
                }
            }

            true
        },
    },
    WeirdYaku {
        name: "Kokushi musou",
        desc: "13 terminals/honors + a copy of one",
        han: 99,
        open_score: OpenScore::Illegal,
        f: |tiles| {
            let mut pair_idx: Option<[usize; 2]> = None;

            for (i, t) in tiles.iter().enumerate() {
                if t.simple() {
                    return false;
                }

                for (j, _) in tiles.iter().enumerate().filter(|(j, o)| *j != i && *o == t) {
                    match pair_idx {
                        Some([a, b]) if j == a || j == b => (),
                        Some(_) => return false,
                        None => pair_idx = Some([i, j]),
                    }
                }
            }

            pair_idx.is_some()
        },
    },
];
