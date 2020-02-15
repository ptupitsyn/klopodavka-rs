pub mod models {
    pub enum Player {
        Red,
        Blue,
    }

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
