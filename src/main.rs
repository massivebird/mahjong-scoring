use self::{
    mentsu::Mentsu,
    player_state::{PlayerState, Wind},
    score::fu,
    yaku::{REGULAR_YAKU, Yaku},
};

mod mentsu;
mod parser;
mod player_state;
mod score;
mod tile;
mod yaku;

fn main() {
    let s = "111222333s22m11z1z";

    let player = player_state::PlayerState {
        seat_wind: Wind::South,
        round_wind: Wind::East,
        dealer: false,
    };

    let parser::Hand {
        tiles,
        win_tile,
        win_method,
    } = parser::parse(s);

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
        .map(|hand| valid_yaku(hand, player))
        .max_by_key(|y| y.iter().map(|y| y.han(true)).sum::<u32>())
        .unwrap()
    {
        println!("{}", yaku.name);
    }
}

fn valid_yaku(hand: &[Mentsu], player: PlayerState) -> Vec<&Yaku> {
    let mut ans = Vec::new();

    for y in REGULAR_YAKU {
        if y.valid_for(hand, player) {
            ans.push(y);
        }
    }

    ans
}
