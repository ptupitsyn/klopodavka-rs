#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
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

    pub fn is_empty(self) -> bool {
        matches!(self, Tile::Empty)
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TilePos {
    pub tile: Tile,
    pub pos: Pos,
}

#[cfg(test)]
mod tests {
    use crate::models::Player;

    #[test]
    fn get_other_player_returns_opponent() {
        let blue = Player::Blue;
        let red = Player::Red;

        assert_eq!(blue.other(), Player::Red);
        assert_eq!(red.other(), Player::Blue);
    }
}
