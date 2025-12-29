use crate::mentsu::Mentsu;

mod regular;
mod weird;
mod yakuman;
mod conditional;

// Export statics under this scope
pub use regular::REGULAR_YAKU;
pub use weird::WEIRD_YAKU;
pub use yakuman::YAKUMAN;
pub use conditional::CONDITIONAL_YAKU;

pub struct Yaku {
    pub name: &'static str,
    pub desc: &'static str,
    pub han: u32,
    pub open_score: OpenScore,
    pub f: fn(&[Mentsu]) -> bool,
}

impl Yaku {
    pub const fn name(&self) -> &str {
        self.name
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
