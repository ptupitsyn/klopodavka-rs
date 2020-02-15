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
}

pub mod board {
    use crate::models::*;

    pub fn base_pos(p: Player) -> (usize, usize) {
        match p {
            Player::Red => (BOARD_WIDTH - BASE_OFFSET - 1, BASE_OFFSET),
            Player::Blue => (BASE_OFFSET, BOARD_HEIGHT - BASE_OFFSET - 1)
        }
    }

    fn place_base(p: Player, t: &mut Tiles) {
        let (x, y) = base_pos(p);
        t[x][y] = Tile::Base(p);
    }

    pub fn create_board() -> Tiles {
        let mut res = [[Tile::Empty; BOARD_WIDTH]; BOARD_HEIGHT];

        place_base(Player::Red, &mut res);
        place_base(Player::Blue, &mut res);

        res
    }
}

fn main() {
    let board = board::create_board();
    println!("Board: {:#?}", board);
}
