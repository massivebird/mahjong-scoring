use std::hash::Hash;

use crate::suit::Suit;

#[derive(Debug, Copy, Clone, Eq, PartialOrd, Ord)]
pub struct Tile {
    pub value: u32,
    pub suit: Suit,
    pub win_method: Option<WinMethod>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit
    }
}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.suit.hash(state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WinMethod {
    Tsumo,
    Ron,
}

impl Tile {
    pub const fn new(value: u32, suit: Suit, win_method: Option<WinMethod>) -> Self {
        Self {
            value,
            suit,
            win_method,
        }
    }

    /// Returns `true` if both tiles can appear in the same sequence.
    pub fn can_sequence(self, b: Self) -> bool {
        self.suit != Suit::Honor
            && self.suit == b.suit
            && self.value != b.value
            && self.value.abs_diff(b.value) <= 2
    }

    pub fn honor(self) -> bool {
        self.suit == Suit::Honor
    }

    pub fn terminal(self) -> bool {
        self.suit != Suit::Honor && matches!(self.value, 1 | 9)
    }

    pub fn simple(self) -> bool {
        !self.terminal()
    }
}
