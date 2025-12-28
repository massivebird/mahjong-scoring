use super::{OpenScore, Yaku};
use crate::{mentsu::kind::Kind, suit::Suit, win_wait::WinWait};
use strum::IntoEnumIterator;

pub static REGULAR_YAKU: &[Yaku] = &[
    Yaku {
        name: "Tanyao",
        desc: "All simples",
        han: 1,
        open_score: OpenScore::Illegal,
        f: |vec_mn| vec_mn.iter().all(|m| !m.contains_terminal()),
    },
    Yaku {
        name: "Chanta",
        desc: "All mentsu contain at least one terminal or honor",
        han: 2,
        open_score: OpenScore::Reduced,
        f: |vec_mn| vec_mn.iter().all(|m| m.contains_terminal() || m.honor()),
    },
    Yaku {
        name: "Junchan (incompatible w chanta)",
        desc: "All mentsu contain at least one terminal",
        han: 3,
        open_score: OpenScore::Reduced,
        f: |vec_mn| vec_mn.iter().all(|m| m.contains_terminal()),
    },
    Yaku {
        name: "Pinfu",
        desc: "Minimum fu; no triplets, non-yakuhai pair, and ryanmen wait",
        han: 1,
        open_score: OpenScore::Illegal,
        f: |vec_mn| {
            vec_mn.iter().all(|m| !m.triplet()) // No triplets
                && !vec_mn.iter().find(|m| m.pair()).unwrap().honor() // Non-honor*** pair FIX LATER
                && !vec_mn.iter().any(|m| m.open && m.win_wait.is_none()) // Menzenchin
                && vec_mn.iter().any(|m| m.win_wait.is_some_and(|w| w == WinWait::Ryanmen)) // Ryanmen
        },
    },
    Yaku {
        name: "Yakuhai (White Dragon)",
        desc: "White dragon triplet",
        han: 1,
        open_score: OpenScore::Full,
        f: |vec_mn| {
            vec_mn.iter().any(|m| {
                if let Kind::Triplet(t) = m.kind {
                    return t.honor() && t.value == 5;
                }

                false
            })
        },
    },
    Yaku {
        name: "Yakuhai (Green Dragon)",
        desc: "Green dragon triplet",
        han: 1,
        open_score: OpenScore::Full,
        f: |vec_mn| {
            vec_mn.iter().any(|m| {
                if let Kind::Triplet(t) = m.kind {
                    return t.honor() && t.value == 6;
                }

                false
            })
        },
    },
    Yaku {
        name: "Yakuhai (Red Dragon)",
        desc: "Red dragon triplet",
        han: 2,
        open_score: OpenScore::Full,
        f: |vec_mn| {
            vec_mn.iter().any(|m| {
                if let Kind::Triplet(t) = m.kind {
                    return t.honor() && t.value == 7;
                }

                false
            })
        },
    },
    Yaku {
        name: "Sanankou",
        desc: "Three concealed triplets",
        han: 2,
        open_score: OpenScore::Full,
        f: |vec_mn| vec_mn.iter().filter(|m| m.closed() && m.triplet()).count() >= 3,
    },
    Yaku {
        name: "Ryanpeikou",
        desc: "Twin identical sequences",
        han: 3,
        open_score: OpenScore::Illegal,
        f: |vec_mn| {
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
        },
    },
    Yaku {
        name: "Iipeikou",
        desc: "Identical sequences",
        han: 1,
        open_score: OpenScore::Illegal,
        f: |vec_mn| {
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
        },
    },
    Yaku {
        name: "Sanshoku doukou",
        desc: "Three colors, same triplet",
        han: 2,
        open_score: OpenScore::Full,
        f: |vec_mn| {
            for (i, m) in vec_mn.iter().enumerate().filter(|(_, m)| m.triplet()) {
                if vec_mn
                    .iter()
                    .enumerate()
                    .filter(|(j, n)| *j != i && m.eq_diff_suits(**n))
                    .count()
                    >= 2
                {
                    return true;
                }
            }

            false
        },
    },
    Yaku {
        name: "Sanshoku doujun",
        desc: "Three colors, same sequence",
        han: 2,
        open_score: OpenScore::Reduced,
        f: |vec_mn| {
            for (i, m) in vec_mn.iter().enumerate().filter(|(_, m)| m.sequence()) {
                if vec_mn
                    .iter()
                    .enumerate()
                    .filter(|(j, n)| *j != i && m.eq_diff_suits(**n))
                    .count()
                    >= 2
                {
                    return true;
                }
            }

            false
        },
    },
    Yaku {
        name: "Toitoi",
        desc: "All triplets",
        han: 2,
        open_score: OpenScore::Full,
        f: |vec_mn| vec_mn.iter().all(|m| !m.sequence()),
    },
    Yaku {
        name: "Ittsuu",
        desc: "Pure straight",
        han: 2,
        open_score: OpenScore::Reduced,
        f: |vec_mn| {
            #[allow(unused)] // Isn't it used??
            let mut seqs = 0;

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
        },
    },
];
