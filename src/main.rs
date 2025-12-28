use self::{
    mentsu::Mentsu,
    score::fu,
    yaku::{REGULAR_YAKU, Yaku},
};

mod i13s;
mod mentsu;
mod parser;
mod score;
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
    let s = "444m444p444s123m5p5p";

    let (i13s, _win_tile, win_method) = parser::parse(s);

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

fn valid_yaku(hand: &[Mentsu]) -> Vec<&Yaku> {
    let mut ans = Vec::new();

    for y in REGULAR_YAKU {
        if y.valid_for(hand) {
            ans.push(y);
        }
    }

    ans
}
