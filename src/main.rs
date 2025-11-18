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
}

fn main() {
    let s = "123m45s";

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

    is_winning(tiles);
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

fn is_winning(tiles: Vec<Tile>) -> u32 {
    let mut score = 0;

    let mut iter = tiles.iter().peekable();

    for (i, tile) in tiles.iter().enumerate() {
        if sequence_at(&tiles, i) {
            println!("Sequence! Starts at i={i}, tile={tile:?}");
        }
    }

    score
}

fn sequence_at(tiles: &[Tile], i: usize) -> bool {
    // Do we have enough tiles to evaluate?
    if tiles.iter().skip(i).take(3).count() != 3 {
        return false;
    }

    let tiles = [tiles[i], tiles[i + 1], tiles[i + 2]];

    tiles[1].value() == tiles[0].value() + 1 && tiles[2].value() == tiles[1].value() + 1
}

fn triplet(tiles: [Tile; 3]) -> bool {
    tiles[0] == tiles[1] && tiles[1] == tiles[2]
}

fn pair(tiles: [Tile; 2]) -> bool {
    tiles[0] == tiles[1]
}
