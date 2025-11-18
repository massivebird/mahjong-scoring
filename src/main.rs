use std::mem::{Discriminant, discriminant};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Man(u32),
    Pin(u32),
    Sou(u32),
}

impl Tile {
    const fn value(self) -> u32 {
        match self {
            Self::Man(val) | Self::Pin(val) | Self::Sou(val) => val,
        }
    }

    const fn suit(self) -> Discriminant<Self> {
        discriminant(&self)
    }
}

fn main() {
    let s = "123444m456s";

    let mut suit_vals: Vec<u32> = Vec::new();
    let mut tiles: Vec<Tile> = Vec::new();

    for c in s.chars() {
        if c.is_ascii_digit() {
            suit_vals.push(c.to_digit(10).unwrap());
            continue;
        }

        for v in &suit_vals {
            tiles.push(to_tile(*v, c).unwrap());
        }

        suit_vals.clear();
    }

    dbg!(&tiles);

    is_winning(&tiles);
}

fn to_tile(val: u32, c: char) -> Result<Tile, String> {
    assert!(val <= 9, "I don't make tiles outside 0..=9. (val={val})");

    match (val, c) {
        (val, 's') => Ok(Tile::Sou(val)),
        (val, 'm') => Ok(Tile::Man(val)),
        (val, 'p') => Ok(Tile::Pin(val)),
        (val, c) => panic!("Unknown tile! v={val}, c={c}"),
    }
}

fn is_winning(tiles: &[Tile]) -> u32 {
    let mut score = 0;

    let mut i = 0;
    while let Some(tile) = tiles.get(i) {
        if sequence_at(tiles, i) {
            println!("Sequence! Starts at i={i}, tile={tile:?}");
            i += 2;
            continue;
        } else if triplet_at(tiles, i) {
            println!("Triplet! Starts at i={i}, tile={tile:?}");
            i += 2;
            continue;
        }

        i += 1;
    }

    score
}

fn sequence_at(tiles: &[Tile], i: usize) -> bool {
    // Do we have enough tiles to evaluate?
    if tiles.iter().skip(i).take(3).count() != 3 {
        return false;
    }

    let tiles = [tiles[i], tiles[i + 1], tiles[i + 2]];

    if tiles[0].suit() != tiles[1].suit() || tiles[1].suit() != tiles[2].suit() {
        return false;
    }

    tiles[1].value() == tiles[0].value() + 1 && tiles[2].value() == tiles[1].value() + 1
}

fn triplet_at(tiles: &[Tile], i: usize) -> bool {
    // Do we have enough tiles to evaluate?
    if tiles.iter().skip(i).take(3).count() != 3 {
        return false;
    }

    let tiles = [tiles[i], tiles[i + 1], tiles[i + 2]];

    if tiles[0].suit() != tiles[1].suit() || tiles[1].suit() != tiles[2].suit() {
        return false;
    }

    tiles[0] == tiles[1] && tiles[1] == tiles[2]
}

fn pair(tiles: [Tile; 2]) -> bool {
    tiles[0] == tiles[1]
}
