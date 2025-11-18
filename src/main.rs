use std::mem::{Discriminant, discriminant};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Man(u32),
    Pin(u32),
    Sou(u32),
    Wind(u32),
}

impl Tile {
    const fn value(self) -> u32 {
        match self {
            Self::Man(val) | Self::Pin(val) | Self::Sou(val) | Self::Wind(val) => val,
        }
    }

    const fn suit(self) -> Discriminant<Self> {
        discriminant(&self)
    }
}

fn main() {
    let s = "123m444456s777s22m";

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

    if is_winning(&tiles) {
        println!("Hand {s} is a winning hand");
    } else {
        println!("Hand {s} is not a winning hand");
    }
}

fn to_tile(val: u32, c: char) -> Result<Tile, String> {
    assert!(val <= 9, "I don't make tiles outside 0..=9. (val={val})");

    match (val, c) {
        (val, 's') => Ok(Tile::Sou(val)),
        (val, 'm') => Ok(Tile::Man(val)),
        (val, 'p') => Ok(Tile::Pin(val)),
        (val, 'z') => Ok(Tile::Wind(val)),
        (val, c) => panic!("Unknown tile! v={val}, c={c}"),
    }
}

fn is_winning(tiles: &[Tile]) -> bool {
    let mut num_trip_seq = 0;
    let mut num_pairs = 0;

    let mut i = 0;
    while let Some(tile) = tiles.get(i) {
        if sequence_at(tiles, i) {
            println!("Sequence! Starts at i={i}, tile={tile:?}");
            num_trip_seq += 1;
            i += 3;
            continue;
        } else if triplet_at(tiles, i) {
            println!("Triplet! Starts at i={i}, tile={tile:?}");
            num_trip_seq += 1;
            i += 3;
            continue;
        } else if pair_at(tiles, i) {
            println!("Pair! Starts at i={i}, tile={tile:?}");
            num_pairs += 1;
            i += 2;
            continue;
        }

        i += 1;
    }

    num_trip_seq == 4 && num_pairs == 1
}

fn sequence_at(tiles: &[Tile], i: usize) -> bool {
    // Do we have enough tiles to evaluate?
    if tiles.iter().skip(i).take(3).count() != 3 {
        return false;
    }

    let tiles = [tiles[i], tiles[i + 1], tiles[i + 2]];

    // Honor/winds cannot form sequences.
    if !tiles
        .iter()
        .all(|t| matches!(t, Tile::Man(_) | Tile::Sou(_) | Tile::Pin(_)))
    {
        return false;
    }

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

fn pair_at(tiles: &[Tile], i: usize) -> bool {
    // Do we have enough tiles to evaluate?
    if tiles.iter().skip(i).take(2).count() != 2 {
        return false;
    }

    let tiles = [tiles[i], tiles[i + 1]];

    tiles[0].value() == tiles[1].value() && tiles[0].suit() == tiles[1].suit()
}
