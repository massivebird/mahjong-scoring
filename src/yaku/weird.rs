use crate::mentsu::get_tiles;
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
            let mut found_pair = false;

            for (i, t) in tiles.iter().enumerate() {
                if t.simple() {
                    return false;
                }

                match tiles
                    .iter()
                    .enumerate()
                    .filter(|(j, o)| *j != i && *o == t)
                    .count()
                {
                    1 => {
                        if found_pair {
                            return false;
                        }
                        found_pair = true;
                    }
                    0 => (),
                    _ => return false,
                }
            }

            found_pair
        },
    },
];
