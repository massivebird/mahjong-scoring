#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Suit {
    Man,
    Pin,
    Sou,
    Honor,
}

impl From<char> for Suit {
    fn from(value: char) -> Self {
        match value {
            'm' => Self::Man,
            's' => Self::Sou,
            'p' => Self::Pin,
            'z' => Self::Honor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile {
    value: u32,
    suit: Suit,
}

impl Tile {
    /// Returns `true` if both tiles can appear in the same sequence.
    fn can_sequence(self, b: Self) -> bool {
        self.suit != Suit::Honor
            && self.suit == b.suit
            && self.value != b.value
            && self.value.abs_diff(b.value) <= 2
    }
}

impl Tile {
    const fn new(value: u32, suit: Suit) -> Self {
        Self { value, suit }
    }
}

#[derive(Debug, Copy, Clone)]
enum Mentsu {
    Triplet(Tile),
    Quad(Tile),
    Sequence(Tile, Tile, Tile),
    Pair(Tile),
}

fn main() {
    let s = "22234m567s";

    let mut suit_vals: Vec<u32> = Vec::new();
    let mut hand_tiles: Vec<Tile> = Vec::new();

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

    dbg!(&hand_tiles);

    let ans = build_mentsu(&hand_tiles, 0, &[]);

    dbg!(&ans);

    println!("{:?} hand interpretations found.", ans.len());
}

fn build_mentsu(as_tiles: &[Tile], i: usize, mentsu_rn: &[Mentsu]) -> Vec<Vec<Mentsu>> {
    let mut ans: Vec<Vec<Mentsu>> = vec![];

    // Exhausted all tiles.
    if i >= as_tiles.len() {
        return vec![mentsu_rn.to_vec()];
    }

    let this = as_tiles[i];

    // Pair
    if i <= as_tiles.len() - 2 && this == as_tiles[i + 1] {
        for m in build_mentsu(as_tiles, i + 2, &with(mentsu_rn, Mentsu::Pair(this))) {
            ans.push(m);
        }
    }

    // Triplet
    if i <= as_tiles.len() - 3 && this == as_tiles[i + 1] && this == as_tiles[i + 2] {
        for m in build_mentsu(as_tiles, i + 3, &with(mentsu_rn, Mentsu::Triplet(this))) {
            ans.push(m);
        }

        // Sure let's check quads in here too
        if i <= as_tiles.len() - 4 && this == as_tiles[i + 3] {
            for m in build_mentsu(as_tiles, i + 4, &with(mentsu_rn, Mentsu::Quad(this))) {
                ans.push(m);
            }
        }
    }

    // Sequence
    if i <= as_tiles.len() - 3
        && this.can_sequence(as_tiles[i + 1])
        && this.can_sequence(as_tiles[i + 2])
    {
        for m in build_mentsu(
            as_tiles,
            i + 3,
            &with(
                mentsu_rn,
                Mentsu::Sequence(this, as_tiles[i + 1], as_tiles[i + 2]),
            ),
        ) {
            ans.push(m);
        }
    }

    ans
}

fn with(vec: &[Mentsu], val: Mentsu) -> Vec<Mentsu> {
    [vec, &[val]].concat()
}

// fn to_tile(val: u32, c: char) -> Result<Suit, String> {
//     assert!(val <= 9, "I don't make tiles outside 0..=9. (val={val})");

//     match (val, c) {
//         (val, 's') => Ok(Suit::Sou(val)),
//         (val, 'm') => Ok(Suit::Man(val)),
//         (val, 'p') => Ok(Suit::Pin(val)),
//         (val, 'z') => Ok(Suit::Honor(val)),
//         (val, c) => panic!("Unknown tile! v={val}, c={c}"),
//     }
// }

// fn is_winning(tiles: &[Suit]) -> bool {
//     let mut num_trip_seq = 0;
//     let mut num_pairs = 0;

//     let mut i = 0;
//     while let Some(tile) = tiles.get(i) {
//         if sequence_at(tiles, i) {
//             println!("Sequence! Starts at i={i}, tile={tile:?}");
//             num_trip_seq += 1;
//             i += 3;
//             continue;
//         } else if triplet_at(tiles, i) {
//             println!("Triplet! Starts at i={i}, tile={tile:?}");
//             num_trip_seq += 1;
//             i += 3;
//             continue;
//         } else if pair_at(tiles, i) {
//             println!("Pair! Starts at i={i}, tile={tile:?}");
//             num_pairs += 1;
//             i += 2;
//             continue;
//         }

//         i += 1;
//     }

//     num_trip_seq == 4 && num_pairs == 1
// }

// fn all_simples(hand: &[Suit]) -> bool {
//     hand.iter().all(|t| {
//         matches!(t.value(), 2..=8) && matches!(t, Suit::Man(_) | Suit::Sou(_) | Suit::Pin(_))
//     })
// }

// fn sequence_at(tiles: &[Suit], i: usize) -> bool {
//     // Do we have enough tiles to evaluate?
//     if tiles.iter().skip(i).take(3).count() != 3 {
//         return false;
//     }

//     let tiles = [tiles[i], tiles[i + 1], tiles[i + 2]];

//     // Honor/winds cannot form sequences.
//     if !tiles
//         .iter()
//         .all(|t| matches!(t, Suit::Man(_) | Suit::Sou(_) | Suit::Pin(_)))
//     {
//         return false;
//     }

//     if tiles[0].suit() != tiles[1].suit() || tiles[1].suit() != tiles[2].suit() {
//         return false;
//     }

//     tiles[1].value() == tiles[0].value() + 1 && tiles[2].value() == tiles[1].value() + 1
// }

// fn triplet_at(tiles: &[Suit], i: usize) -> bool {
//     // Do we have enough tiles to evaluate?
//     if tiles.iter().skip(i).take(3).count() != 3 {
//         return false;
//     }

//     let tiles = [tiles[i], tiles[i + 1], tiles[i + 2]];

//     if tiles[0].suit() != tiles[1].suit() || tiles[1].suit() != tiles[2].suit() {
//         return false;
//     }

//     tiles[0] == tiles[1] && tiles[1] == tiles[2]
// }

// fn pair_at(tiles: &[Suit], i: usize) -> bool {
//     // Do we have enough tiles to evaluate?
//     if tiles.iter().skip(i).take(2).count() != 2 {
//         return false;
//     }

//     let tiles = [tiles[i], tiles[i + 1]];

//     tiles[0].value() == tiles[1].value() && tiles[0].suit() == tiles[1].suit()
// }
