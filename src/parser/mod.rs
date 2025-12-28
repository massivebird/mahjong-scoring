use crate::mentsu::Mentsu;
use crate::suit::Suit;
use crate::tile::Tile;
use crate::{WinMethod};

mod i13s;

pub fn parse(s: &str) -> (Vec<Vec<Mentsu>>, Tile, WinMethod) {
    let mut suit_vals: Vec<u32> = Vec::new();
    let mut hand_tiles: Vec<Tile> = Vec::new();

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
            hand_tiles.push(Tile::new(*val, Suit::from(c)));
        }

        suit_vals.clear();
    }

    hand_tiles.sort();

    let i13s = i13s::build(&hand_tiles, win_tile, win_method);

    (i13s, win_tile, win_method)
}
