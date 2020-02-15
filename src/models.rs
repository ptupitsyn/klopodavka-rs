enum Player {
    Red,
    Blue,
}

enum Tile {
    Empty,
    Base(Player),
    Alive(Player),
    Squashed(Player),
}

const BOARD_SIZE: usize = 30;

type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];

struct GameState {
    board: Tiles,
    current_player: Player,
    turn_length: u32,
    moves_left: u32,
}
