#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Base(Player),
    Alive(Player),
    Squashed(Player),
}

impl Tile {
    pub fn is_alive(self) -> bool {
        matches!(self, Tile::Alive(_))
    }
}

pub const BOARD_WIDTH: usize = 8;

pub const BOARD_HEIGHT: usize = 10;

pub const BASE_OFFSET: usize = 2;

// TODO: This should probably be dynamic in size (vec) for customizable boards
pub type Tiles = [[Tile; BOARD_HEIGHT]; BOARD_WIDTH];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TilePos {
    pub tile: Tile,
    pub pos: Pos,
}
