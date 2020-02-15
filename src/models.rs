#[derive(Debug, Copy, Clone)]
pub enum Player {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Base(Player),
    Alive(Player),
    Squashed(Player),
}

pub const BOARD_WIDTH: usize = 10;

pub const BOARD_HEIGHT: usize = 10;

pub const BASE_OFFSET: usize = 2;

pub type Tiles = [[Tile; BOARD_WIDTH]; BOARD_HEIGHT];

pub struct GameState {
    board: Tiles,
    current_player: Player,
    turn_length: u32,
    moves_left: u32,
}
