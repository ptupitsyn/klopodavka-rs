use std::ops::{Index, IndexMut};

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

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

/// Board size alias.
pub type Bsize = u16;

pub const BASE_OFFSET: Bsize = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct Tiles<T: Default + Copy> {
    tiles: Vec<T>,
    size: Size,
}

impl<T: Default + Copy> Tiles<T> {
    pub fn with_size(size: Size) -> Self {
        Tiles::with_size_and_val(size, T::default())
    }

    pub fn with_size_and_val(size: Size, val: T) -> Self {
        let min = BASE_OFFSET * 2;
        if size.width < min || size.height < min {
            panic!(
                "Board size is too small: {} by {}. Should be at least {} by {}",
                size.width, size.height, min, min
            );
        }

        Tiles {
            size,
            tiles: (0..size.width * size.height).map(|_| val).collect(),
        }
    }

    pub fn get(&self, x: Bsize, y: Bsize) -> Option<T> {
        self.idx(x, y).map(|i| self.tiles[i])
    }

    pub fn getp(&self, pos: Pos) -> Option<T> {
        self.get(pos.x, pos.y)
    }

    pub fn set(&mut self, x: Bsize, y: Bsize, tile: T) {
        let idx = self.idx(x, y).expect("Invalid position");
        self.tiles[idx] = tile
    }

    #[inline]
    pub fn setp(&mut self, pos: Pos, tile: T) {
        self.set(pos.x, pos.y, tile)
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    #[inline]
    fn idx(&self, x: Bsize, y: Bsize) -> Option<usize> {
        if x < self.size.width && y < self.size.height {
            let index = self.size.width * y + x;

            Some(index as usize)
        } else {
            None
        }
    }
}

impl<T: Default + Copy> Index<Pos> for Tiles<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        let idx = self.idx(index.x, index.y).expect("valid pos");
        &self.tiles[idx]
    }
}

impl<T: Default + Copy> IndexMut<Pos> for Tiles<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        let idx = self.idx(index.x, index.y).expect("valid pos");
        &mut self.tiles[idx]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: Bsize,
    pub y: Bsize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: Bsize,
    pub height: Bsize,
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
