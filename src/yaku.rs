use crate::mentsu::Mentsu;

pub struct Yaku {
    name: String,
    desc: String,
    han: u32,
    open_score: OpenScore,
    f: Box<dyn Fn(&[Mentsu]) -> bool>,
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
            name: "Pinfu (WIP)".to_string(),
            desc: "Minimum fu; no triplets, non-honor pair, and ryanmen wait".to_string(),
            han: 1,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| {
                vec_mn.iter().all(|m| !m.triplet())
                    && !vec_mn.iter().find(|m| m.pair()).unwrap().honor()
            }),
        },
        Yaku {
            name: "Toitoi".to_string(),
            desc: "All triplets".to_string(),
            han: 2,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| !m.sequence())),
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
