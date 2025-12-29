use super::{OpenScore, Yaku};

pub static CONDITIONAL_YAKU: &[Yaku] = &[
    Yaku {
        name: "Riichi",
        desc: "A closed hand declaring tenpai. Costs 1,000 pts.",
        han: 1,
        open_score: OpenScore::Illegal,
        f: |_, _| true,
    },
    Yaku {
        name: "Double riichi",
        desc: "Declared riichi on first turn before a tile call.",
        han: 2,
        open_score: OpenScore::Illegal,
        f: |_, _| true,
    },
    Yaku {
        name: "Ippatsu",
        desc: "Won with riichi before your next discard. Invalidated by calls.",
        han: 1,
        open_score: OpenScore::Illegal,
        f: |_, _| true,
    },
    Yaku {
        name: "Rinshan kaihou",
        desc: "Won by drawing a tile from the dead wall.",
        han: 1,
        open_score: OpenScore::Illegal,
        f: |_, _| true,
    },
    Yaku {
        name: "Haitei/houtei",
        desc: "Won by drawing or calling the final tile from the wall.",
        han: 1,
        open_score: OpenScore::Illegal,
        f: |_, _| true,
    },
    Yaku {
        name: "Chankan",
        desc: "Won by calling ron on an added kan.",
        han: 1,
        open_score: OpenScore::Full,
        f: |_, _| true,
    },
];
