enum Player {
    Red,
    Blue
}

enum Tile {
    Empty,
    Base(Player),
    Alive(Player),
    Squashed(Player)
}