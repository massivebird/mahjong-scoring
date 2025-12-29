#[derive(Copy, Clone, Debug)]
pub struct PlayerState {
    pub seat_wind: Wind,
    pub round_wind: Wind,
    pub dealer: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

