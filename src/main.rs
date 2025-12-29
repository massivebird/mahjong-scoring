use self::{
    mentsu::Mentsu,
    score::fu,
    yaku::{REGULAR_YAKU, Yaku},
};

mod mentsu;
mod parser;
mod score;
mod tile;
mod yaku;

fn main() {
    let s = "111222333s22m46m5m";

    let (tiles, win_tile, win_method) = parser::parse(s);
    let i13s = parser::interpret(&tiles, win_tile, win_method);

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

    println!("Weirdos?");
    for w in yaku::WEIRD_YAKU {
        if (w.f)(&tiles) {
            println!("{}", w.name);
        }
    }

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
