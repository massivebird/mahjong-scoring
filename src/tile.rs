use crate::suit::Suit;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Eq, Ord)]
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

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(core::cmp::Ordering::Equal) => (),
            ord => return ord,
        }
        match self.suit.partial_cmp(&other.suit) {
            Some(core::cmp::Ordering::Equal) => return Some(core::cmp::Ordering::Equal),
            ord => return ord,
        }
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

    pub const fn add(self, rhs: u32) -> Option<Self> {
        let value = self.value + rhs;

        if value > 9 {
            None
        } else {
            Some(Self {
                value,
                suit: self.suit,
                win_method: None,
            })
        }
    }
}
