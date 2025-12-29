use super::{OpenScore, Yaku};

pub static YAKUMAN: &[Yaku] = &[
    Yaku {
        name: "Tsuuiisou",
        desc: "All honors",
        han: 99,
        open_score: OpenScore::Full,
        f: |vec_mn, _state| vec_mn.iter().all(|m| m.honor()),
    },
    Yaku {
        name: "Chinroutou",
        desc: "All terminals",
        han: 99,
        open_score: OpenScore::Full,
        f: |vec_mn, _state| vec_mn.iter().all(|m| m.entirely_terminal()),
    },
    Yaku {
        name: "Suuankou",
        desc: "Four concealed triplets",
        han: 99,
        open_score: OpenScore::Illegal,
        f: |vec_mn, _state| vec_mn.iter().filter(|m| m.closed() && m.triplet()).count() >= 4,
    },
];
