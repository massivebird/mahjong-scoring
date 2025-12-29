use crate::mentsu::Mentsu;
use crate::tile::Suit;
use crate::tile::Tile;

mod i13s;
mod win_method;
mod win_wait;

pub use win_method::WinMethod;
pub use win_wait::WinWait;

pub struct Hand {
    pub tiles: Vec<Tile>,
    pub win_tile: Tile,
    pub win_method: WinMethod,
}

pub fn parse(s: &str) -> Hand {
    let mut suit_vals: Vec<u32> = Vec::new();
    let mut tiles: Vec<Tile> = Vec::new();

    let (win_tile, win_method) = {
        let t = Tile::new(
            s.chars().nth(s.len() - 2).unwrap().to_digit(10).unwrap(),
            Suit::from(s.chars().nth(s.len() - 1).unwrap()),
        );

        match s.chars().nth(s.len() - 3) {
            Some(c) if c.is_whitespace() => (t, WinMethod::Ron),
            _ => (t, WinMethod::Tsumo),
        }
    };

    dbg!((win_tile, win_method));

    for c in s.chars() {
        if c.is_ascii_digit() {
            suit_vals.push(c.to_digit(10).unwrap());
            continue;
        }

        for val in &suit_vals {
            tiles.push(Tile::new(*val, Suit::from(c)));
        }

        suit_vals.clear();
    }

    tiles.sort();

    Hand {
        tiles,
        win_tile,
        win_method,
    }
}

pub fn interpret(hand_tiles: &[Tile], win_tile: Tile, win_method: WinMethod) -> Vec<Vec<Mentsu>> {
    let i13s = i13s::build(&hand_tiles, win_tile, win_method);

    i13s
}
