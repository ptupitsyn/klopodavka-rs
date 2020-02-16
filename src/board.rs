use crate::models::*;

pub fn base_pos(p: Player) -> (usize, usize) {
    match p {
        Player::Red => (BASE_OFFSET, BOARD_HEIGHT - BASE_OFFSET - 1),
        Player::Blue => (BOARD_WIDTH - BASE_OFFSET - 1, BASE_OFFSET),
    }
}

pub fn create_board() -> Tiles {
    let mut res = [[Tile::Empty; BOARD_HEIGHT]; BOARD_WIDTH];

    fn place_base(p: Player, t: &mut Tiles) {
        let (x, y) = base_pos(p);
        t[x][y] = Tile::Base(p);
    }

    place_base(Player::Red, &mut res);
    place_base(Player::Blue, &mut res);

    res
}

pub fn get_other_player(player: Player) -> Player {
    match player {
        Player::Red => Player::Blue,
        Player::Blue => Player::Red,
    }
}

pub fn make_move(board: &mut Tiles, player: Player, x: usize, y: usize) {
    let tile = board[x][y];
    let other_player = get_other_player(player);

    board[x][y] =
        match tile {
            Tile::Empty => Tile::Alive(player),
            Tile::Alive(other_player) => Tile::Squashed(player),
            _ => { panic!("Invalid move") }
        }
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::models::*;

    #[test]
    fn create_board_returns_empty_board_with_bases() {
        let board = create_board();
        let (rx, ry) = base_pos(Player::Red);
        let (bx, by) = base_pos(Player::Blue);

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let tile = board[x][y];
                if x == rx && y == ry {
                    assert_eq!(Tile::Base(Player::Red), tile);
                } else if x == bx && y == by {
                    assert_eq!(Tile::Base(Player::Blue), tile);
                } else {
                    assert_eq!(Tile::Empty, tile);
                }
            }
        }
    }

    #[test]
    fn get_other_player_returns_opponent() {
        assert_eq!(Player::Red, get_other_player(Player::Blue));
        assert_eq!(Player::Blue, get_other_player(Player::Red));
    }
}