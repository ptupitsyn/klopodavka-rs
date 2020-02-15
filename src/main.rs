pub mod models {
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

    pub const BOARD_SIZE: usize = 30;

    pub type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];

    pub struct GameState {
        board: Tiles,
        current_player: Player,
        turn_length: u32,
        moves_left: u32,
    }
}

pub mod board {
    use crate::models::*;

    pub fn create_board() -> Tiles {
        let res = [[Tile::Empty; BOARD_SIZE]; BOARD_SIZE];
        res
    }
}

fn main() {
    let board = board::create_board();
    println!("Hello, world!");
}
