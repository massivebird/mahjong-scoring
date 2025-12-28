use strum::IntoEnumIterator;

use crate::mentsu::{Mentsu, kind::Kind};
use crate::suit::Suit;
use crate::win_wait::WinWait;

pub struct Yaku {
    pub name: String,
    pub desc: String,
    pub han: u32,
    pub open_score: OpenScore,
    pub f: Box<dyn Fn(&[Mentsu]) -> bool>,
}

impl Yaku {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn valid_for(&self, mentsu: &[Mentsu]) -> bool {
        (self.f)(mentsu)
    }
}

pub enum OpenScore {
    Full,
    Reduced,
    Illegal,
}

pub fn regular_yaku() -> Vec<Yaku> {
    vec![
        Yaku {
            name: "Tanyao".to_string(),
            desc: "All simples".to_string(),
            han: 1,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| !m.contains_terminal())),
        },
        Yaku {
            name: "Chanta".to_string(),
            desc: "All mentsu contain at least one terminal or honor".to_string(),
            han: 2,
            open_score: OpenScore::Reduced,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| m.contains_terminal() || m.honor())),
        },
        Yaku {
            name: "Junchan (incompatible w chanta)".to_string(),
            desc: "All mentsu contain at least one terminal".to_string(),
            han: 3,
            open_score: OpenScore::Reduced,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| m.contains_terminal())),
        },
        Yaku {
            name: "Pinfu".to_string(),
            desc: "Minimum fu; no triplets, non-yakuhai pair, and ryanmen wait".to_string(),
            han: 1,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| {
                vec_mn.iter().all(|m| !m.triplet()) // No triplets
                    && !vec_mn.iter().find(|m| m.pair()).unwrap().honor() // Non-honor*** pair FIX LATER
                    && !vec_mn.iter().any(|m| m.open && m.win_wait.is_none()) // Menzenchin
                    && vec_mn.iter().any(|m| m.win_wait.is_some_and(|w| w == WinWait::Ryanmen)) // Ryanmen
            }),
        },
        Yaku {
            name: "Yakuhai (White Dragon)".to_string(),
            desc: "White dragon triplet".to_string(),
            han: 1,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| {
                vec_mn.iter().any(|m| {
                    if let Kind::Triplet(t) = m.kind {
                        return t.honor() && t.value == 5;
                    }

                    false
                })
            }),
        },
        Yaku {
            name: "Yakuhai (Green Dragon)".to_string(),
            desc: "Green dragon triplet".to_string(),
            han: 1,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| {
                vec_mn.iter().any(|m| {
                    if let Kind::Triplet(t) = m.kind {
                        return t.honor() && t.value == 6;
                    }

                    false
                })
            }),
        },
        Yaku {
            name: "Yakuhai (Red Dragon)".to_string(),
            desc: "Red dragon triplet".to_string(),
            han: 2,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| {
                vec_mn.iter().any(|m| {
                    if let Kind::Triplet(t) = m.kind {
                        return t.honor() && t.value == 7;
                    }

                    false
                })
            }),
        },
        Yaku {
            name: "Sanankou".to_string(),
            desc: "Three concealed triplets".to_string(),
            han: 2,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| vec_mn.iter().filter(|m| m.closed() && m.triplet()).count() >= 3),
        },
        Yaku {
            name: "Ryanpeikou".to_string(),
            desc: "Twin identical sequences".to_string(),
            han: 3,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| {
                let mut twin_idx: Option<[usize; 2]> = None;
                for (i, m) in vec_mn.iter().enumerate().filter(|(_, m)| m.sequence()) {
                    // Don't double-match!
                    if twin_idx.is_some_and(|a| a.contains(&i)) {
                        continue;
                    }

                    if let Some((j, _)) = vec_mn
                        .iter()
                        .enumerate()
                        .find(|(j, other)| *j != i && *other == m)
                    {
                        if twin_idx.is_some() {
                            return true;
                        }

                        twin_idx = Some([i, j]);
                    }
                }

                false
            }),
        },
        Yaku {
            name: "Iipeikou".to_string(),
            desc: "Identical sequences".to_string(),
            han: 1,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| {
                for (i, m) in vec_mn.iter().enumerate().filter(|(_, m)| m.sequence()) {
                    if vec_mn
                        .iter()
                        .enumerate()
                        .any(|(j, other)| j != i && other == m)
                    {
                        return true;
                    }
                }

                false
            }),
        },
        Yaku {
            name: "Toitoi".to_string(),
            desc: "All triplets".to_string(),
            han: 2,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| !m.sequence())),
        },
        Yaku {
            name: "Ittsuu".to_string(),
            desc: "Pure straight".to_string(),
            han: 2,
            open_score: OpenScore::Reduced,
            f: Box::new(|vec_mn| {
                let mut seqs = 0b000;

                for s in Suit::iter() {
                    seqs = 0;

                    for m in vec_mn.iter().filter(|m| m.suit() == s) {
                        if let Kind::Sequence(f, _, _) = m.kind
                            && f.value == seqs * 3 + 1
                        {
                            seqs += 1;

                            if seqs == 3 {
                                return true;
                            }
                        }
                    }
                }

                false
            }),
        },
    ]
}

pub fn yakuman() -> Vec<Yaku> {
    vec![
        Yaku {
            name: "Tsuuiisou".to_string(),
            desc: "All honors".to_string(),
            han: 99,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| m.honor())),
        },
        Yaku {
            name: "Chinroutou".to_string(),
            desc: "All terminals".to_string(),
            han: 99,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| m.entirely_terminal())),
        },
        Yaku {
            name: "Suuankou".to_string(),
            desc: "Four concealed triplets".to_string(),
            han: 99,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| vec_mn.iter().filter(|m| m.closed() && m.triplet()).count() >= 4),
        },
    ]
}

/// These are tougher to evaluate with regular mentsu parsing.
pub fn weird_yaku_and_yakuman() -> Vec<Yaku> {
    vec![
        Yaku {
            name: "Chiitoi".to_string(),
            desc: "Seven (7) pairs".to_string(),
            han: 2,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| vec_mn.iter().filter(|m| m.pair()).count() == 7),
        },
        Yaku {
            name: "Kokushi musou".to_string(),
            desc: "13 terminals/honors + a copy of one".to_string(),
            han: 99,
            open_score: OpenScore::Illegal,
            f: Box::new(|_| true), // Must be early-evaluated
        },
    ]
}
