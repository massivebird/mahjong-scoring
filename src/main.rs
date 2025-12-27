use self::{mentsu::Mentsu, suit::Suit, tile::Tile, yaku::regular_yaku};

mod mentsu;
mod suit;
mod tile;
mod yaku;

fn main() {
    let s = "123m456m789m123m99m";

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

    let mut i13s = build_mentsu(&hand_tiles, 0, &[]);

    print!("{} total interpretations, ", i13s.len());

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
