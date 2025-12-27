use crate::suit::Suit;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile {
    pub value: u32,
    pub suit: Suit,
}

impl Tile {
    pub const fn new(value: u32, suit: Suit) -> Self {
        Self { value, suit }
    }

    /// Returns `true` if both tiles can appear in the same sequence.
    pub fn can_sequence(self, b: Self) -> bool {
        self.suit != Suit::Honor
            && self.suit == b.suit
            && self.value != b.value
            && self.value.abs_diff(b.value) <= 2
    }

    pub fn terminal(self) -> bool {
        self.suit != Suit::Honor && matches!(self.value, 1 | 9)
    }

    pub fn simple(self) -> bool {
        !self.terminal()
    }
}
