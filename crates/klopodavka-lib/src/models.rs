#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    pub fn other(self) -> Player {
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

    pub fn is_squashed(self) -> bool {
        matches!(self, Tile::Squashed(_))
    }

    pub fn is_empty(self) -> bool {
        matches!(self, Tile::Empty)
    }
}

/// Board size alias.
pub type Bsize = u16;

pub const BASE_OFFSET: Bsize = 2;

#[derive(Debug, Clone)]
pub struct Tiles {
    tiles: Vec<Tile>,
    width: Bsize,
    height: Bsize,
}

impl Tiles {
    pub fn new_default() -> Self {
        Tiles::new(30, 30)
    }

    pub fn new(width: Bsize, height: Bsize) -> Self {
        Tiles {
            width,
            height,
            tiles: [0..width * height].iter().map(|_| Tile::Empty).collect(),
        }
    }

    pub fn get(&self, x: Bsize, y: Bsize) -> Option<Tile> {
        self.index(x, y).map(|i| self.tiles[i])
    }

    pub fn set(&mut self, x: Bsize, y: Bsize, tile: Tile) -> Result<(), ()> {
        match self.index(x, y) {
            None => Result::Err(()),
            Some(idx) => {
                self.tiles[idx] = tile;
                Result::Ok(())
            }
        }
    }

    fn index(&self, x: Bsize, y: Bsize) -> Option<usize> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = self.width * y + x;

            Some(index as usize)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: Bsize,
    pub y: Bsize,
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
