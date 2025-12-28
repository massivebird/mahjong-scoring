use self::{
    mentsu::{Mentsu, kind::Kind},
    suit::Suit,
    tile::Tile,
    win_wait::WinWait,
    yaku::{Yaku, regular_yaku},
};

mod i13s;
mod mentsu;
mod suit;
mod tile;
mod win_wait;
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

    let i13s = i13s::build(&hand_tiles, win_tile, win_method);

    println!("{} winning interpretation(s):", i13s.len());
    for hand in &i13s {
        for m in hand {
            print!("{m}, ");
        }
        println!();
    }

    println!(
        "{} fu",
        i13s.iter().map(|hand| fu(hand, win_method)).max().unwrap()
    );

    println!("Best yaku combo:");
    for yaku in i13s
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
