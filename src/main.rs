use self::tile::WinMethod;
use self::{mentsu::Mentsu, suit::Suit, tile::Tile, yaku::regular_yaku};

mod mentsu;
mod suit;
mod tile;
mod yaku;

fn main() {
    let s = "12367m5m";

    let mut suit_vals: Vec<u32> = Vec::new();
    let mut hand_tiles: Vec<Tile> = Vec::new();

    hand_tiles.push(Tile::new(
        s.chars().nth(s.len() - 2).unwrap().to_digit(10).unwrap(),
        Suit::from(s.chars().nth(s.len() - 1).unwrap()),
        match s.chars().nth(s.len() - 3) {
            Some(c) if c.is_whitespace() => Some(WinMethod::Ron),
            Some(_) => Some(WinMethod::Tsumo),
            _ => None,
        },
    ));

    for c in s.chars().take(s.len() - 2) {
        if c.is_ascii_digit() {
            suit_vals.push(c.to_digit(10).unwrap());
            continue;
        }

        for val in &suit_vals {
            hand_tiles.push(Tile::new(*val, Suit::from(c), None));
        }

        suit_vals.clear();
    }

    hand_tiles.sort();

    let mut i13s = mentsu::build_mentsu(&hand_tiles);

    print!("{} total interpretations, ", i13s.len());

    dbg!(&i13s);

    i13s.retain(|v| {
        v.iter()
            .filter(|m| {
                matches!(
                    m,
                    Mentsu::Triplet(_) | Mentsu::Quad(_) | Mentsu::Sequence(_, _, _)
                )
            })
            .count()
            == 4
            && v.iter().filter(|m| matches!(m, Mentsu::Pair(_))).count() == 1
    });

    println!("{} winning.", i13s.len());

    let yaku = regular_yaku();

    println!("Matching yaku:");
    for y in yaku {
        for m in &i13s {
            if y.valid_for(m.clone()) {
                println!("{}", y.name());
            }
        }
    }
}
