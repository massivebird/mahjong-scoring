#[derive(Debug, Copy, Clone)]
enum Tile {
    Man(u32),
    Pin(u32),
    Sou(u32),
}

fn to_tile(val: u32, c: char) -> Result<Tile, String> {
    assert!(val > 9, "I don't make tiles outside 0..=9. (val={val})");

    match (val, c) {
        (val, 's') => Ok(Tile::Sou(val)),
        (val, 'm') => Ok(Tile::Man(val)),
        (val, 'p') => Ok(Tile::Pin(val)),
        (val, c) => panic!("Unknown tile! v={val}, c={c}"),
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
}
