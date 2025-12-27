use crate::mentsu::Mentsu;

pub struct Yaku {
    name: String,
    han: u32,
    open_score: OpenScore,
    f: Box<dyn Fn(Vec<Mentsu>) -> bool>,
}

impl Yaku {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn valid_for(&self, mentsu: Vec<Mentsu>) -> bool {
        (self.f)(mentsu)
    }
}

pub enum OpenScore {
    Full,
    Reduced,
    Illegal,
}

pub fn generate_yaku() -> Vec<Yaku> {
    vec![
        Yaku {
            name: "Tanyao".to_string(),
            han: 1,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| !m.contains_terminal())),
        },
        Yaku {
            name: "Chanta".to_string(),
            han: 2,
            open_score: OpenScore::Reduced,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| m.contains_terminal() || m.honor())),
        },
        Yaku {
            name: "Pinfu (WIP)".to_string(),
            han: 1,
            open_score: OpenScore::Illegal,
            f: Box::new(|vec_mn| {
                vec_mn.iter().all(|m| !m.triplet())
                    && !vec_mn.iter().find(|m| m.pair()).unwrap().honor()
            }),
        },
        Yaku {
            name: "Toitoi".to_string(),
            han: 2,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| !m.sequence())),
        },
        Yaku {
            name: "Tsuuiisou".to_string(),
            han: 99,
            open_score: OpenScore::Full,
            f: Box::new(|vec_mn| vec_mn.iter().all(|m| m.honor())),
        },
    ]
}
