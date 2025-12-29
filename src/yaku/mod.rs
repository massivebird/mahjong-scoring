use crate::mentsu::Mentsu;
use crate::player_state::PlayerState;

mod conditional;
mod regular;
mod weird;
mod yakuman;

// Export statics under this scope
pub use conditional::CONDITIONAL_YAKU;
pub use regular::REGULAR_YAKU;
pub use weird::WEIRD_YAKU;
pub use yakuman::YAKUMAN;

#[derive(Copy, Clone, Debug)]
pub struct Yaku {
    pub name: &'static str,
    pub desc: &'static str,
    han: u32,
    pub open_score: OpenScore,
    pub f: fn(&[Mentsu], PlayerState) -> bool,
}

impl Yaku {
    pub const fn name(&self) -> &str {
        self.name
    }

    pub fn valid_for(&self, mentsu: &[Mentsu], player: PlayerState) -> bool {
        (self.f)(mentsu, player)
    }

    /// Returns the yaku's han value. If applicable, accounts for reduced han
    /// when scored open.
    pub const fn han(self, menzenchin: bool) -> u32 {
        match (self.open_score, menzenchin) {
            (OpenScore::Full, _) => self.han,
            (OpenScore::Reduced | OpenScore::Illegal, true) => self.han,
            (OpenScore::Reduced, false) => self.han - 1,
            (OpenScore::Illegal, false) => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum OpenScore {
    Full,
    Reduced,
    Illegal,
}
