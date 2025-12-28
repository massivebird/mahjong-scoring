use std::collections::HashSet;

use self::{
    mentsu::{Kind, Mentsu, WinWait},
    suit::Suit,
    tile::Tile,
    yaku::{Yaku, regular_yaku},
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
    let s = "123m56m789m456s77s4m";

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

    println!("{} winning interpretation(s):", v.len());
    for hand in &v {
        for m in hand {
            print!("{m}, ");
        }
        println!();
    }

    println!(
        "{} fu",
        v.iter().map(|hand| fu(hand, win_method)).max().unwrap()
    );

    println!("Best yaku combo:");
    for yaku in v
        .iter()
        .map(|hand| valid_yaku(hand))
        .max_by_key(|y| y.iter().map(|y| y.han).sum::<u32>())
        .unwrap()
    {
        println!("{}", yaku.name);
    }
}

fn valid_yaku(hand: &[Mentsu]) -> Vec<Yaku> {
    let mut ans = Vec::new();

    for y in regular_yaku() {
        if y.valid_for(hand) {
            ans.push(y);
        }
    }

    ans
}

fn fu(hand: &[Mentsu], win_method: WinMethod) -> u32 {
    let menzenchin = !hand.iter().any(|m| m.open && m.win_wait.is_none());

    let mut total = if win_method == WinMethod::Ron && menzenchin {
        30
    } else {
        20
    };

    for m in hand {
        if let Some(wait) = m.win_wait
            && !matches!(wait, WinWait::Ryanmen | WinWait::Shanpon)
        {
            total += 2;
        }

        let mut pts = match m.kind {
            Kind::Triplet(_) => 4,
            Kind::Quad(_) => 16,
            _ => continue,
        };

        if m.open {
            pts /= 2;
        }

        if m.honor() || m.entirely_terminal() {
            pts *= 2;
        }

        total += pts;
    }

    total + (10 - total % 10) // Round to nearest 10
}

fn build_i13s_open(
    i13s: &[Vec<Mentsu>],
    win_tile: Tile,
    win_method: WinMethod,
) -> Vec<Vec<Mentsu>> {
    let mut ans: HashSet<Vec<Mentsu>> = HashSet::new();

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

                // By sorting before inserting, hands are effectively
                // hashed/compared as unordered collections.
                h.sort();
                ans.insert(h);
            }
        }
    }

    ans.into_iter().collect()
}
