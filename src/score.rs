use crate::{
    WinMethod,
    mentsu::{Mentsu, kind::Kind},
    win_wait::WinWait,
};

/// Computes the hand's fu.
pub fn fu(hand: &[Mentsu], win_method: WinMethod) -> u32 {
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

    if total % 10 == 0 {
        total
    } else {
        total + (10 - total % 10) // Round to nearest 10
    }
}
