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

#[derive(Copy, Clone)]
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
    pub fn han(self) -> u32 {
        match self.open_score {
            OpenScore::Full => self.han,
            OpenScore::Reduced => self.han - 1,
            OpenScore::Illegal => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum OpenScore {
    Full,
    Reduced,
    Illegal,
}
