use crate::tile::Tile;

#[derive(Debug, Copy, Clone)]
pub enum Mentsu {
    Triplet(Tile),
    Quad(Tile),
    Sequence(Tile, Tile, Tile),
    Pair(Tile),
}
