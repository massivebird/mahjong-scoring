use super::{OpenScore, Yaku};

/// These are tougher to evaluate with regular mentsu parsing.
pub static WEIRD_YAKU: &[Yaku] = &[
    Yaku {
        name: "Chiitoi",
        desc: "Seven (7) pairs",
        han: 2,
        open_score: OpenScore::Illegal,
        f: |vec_mn| vec_mn.iter().filter(|m| m.pair()).count() == 7,
    },
    Yaku {
        name: "Kokushi musou",
        desc: "13 terminals/honors + a copy of one",
        han: 99,
        open_score: OpenScore::Illegal,
        f: |_| true, // Must be early-evaluated
    },
];
