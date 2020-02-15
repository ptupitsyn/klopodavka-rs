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

const BOARD_SIZE:usize = 30;

type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];
