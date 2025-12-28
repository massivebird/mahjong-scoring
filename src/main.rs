use self::{
    mentsu::{Kind, Mentsu, WinWait},
    suit::Suit,
    tile::Tile,
    yaku::regular_yaku,
};

mod mentsu;
mod suit;
mod tile;
mod yaku;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WinMethod {
    Tsumo,
    Ron,
}

fn main() {
    let s = "123m456m789m123m9m9m";

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

    let mut i13s = mentsu::build_mentsu(&hand_tiles);

    i13s.retain(|v| {
        v.iter()
            .filter(|m| {
                matches!(
                    m.kind,
                    Kind::Triplet(_) | Kind::Quad(_) | Kind::Sequence(_, _, _)
                )
            })
            .count()
            == 4
            && v.iter().filter(|m| matches!(m.kind, Kind::Pair(_))).count() == 1
    });

    let v = build_i13s_open(&i13s, win_tile, win_method);

    dbg!(&v);

    println!("{} winning interpretations.", v.len());

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

fn build_i13s_open(
    i13s: &[Vec<Mentsu>],
    win_tile: Tile,
    win_method: WinMethod,
) -> Vec<Vec<Mentsu>> {
    let mut ans: Vec<Vec<Mentsu>> = Vec::new();

    let mut i13s = i13s.to_owned();

    for hand in &mut i13s {
        for (i, m) in hand.iter().enumerate() {
            if m.contains(win_tile) {
                // Push a copy of this hand with this mentsu as open.
                let mut h = hand.clone();

                if win_method == WinMethod::Ron {
                    h[i].set_open(true);
                }

                let mut set_wait = |wait| h[i].set_win_wait(Some(wait));

                match m.kind {
                    // Tanki
                    Kind::Pair(_) => set_wait(WinWait::Tanki),

                    // Shanpon
                    Kind::Triplet(_) => set_wait(WinWait::Shanpon),

                    // Kanchan
                    Kind::Sequence(_, mid, _) if win_tile == mid => {
                        set_wait(WinWait::Kanchan);
                    }

                    // Penchan
                    Kind::Sequence(left, _, right)
                        if win_tile == left && right.terminal()
                            || win_tile == right && left.terminal() =>
                    {
                        set_wait(WinWait::Penchan);
                    }

                    // Ryanmen
                    Kind::Sequence(_, _, _) => set_wait(WinWait::Ryanmen),

                    // Where do quads fit into this? idk
                    Kind::Quad(_) => unimplemented!(),
                }

                ans.push(h);
            }
        }
    }

    ans
}
