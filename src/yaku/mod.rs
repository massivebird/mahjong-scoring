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

pub struct Yaku {
    pub name: &'static str,
    pub desc: &'static str,
    pub han: u32,
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
}

pub enum OpenScore {
    Full,
    Reduced,
    Illegal,
}
