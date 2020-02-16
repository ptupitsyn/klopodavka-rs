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

pub fn other_player(player: Player) -> Player {
    match player {
        Player::Red => Player::Blue,
        Player::Blue => Player::Red,
    }
}

pub fn make_move(board: &mut Tiles, player: Player, x: usize, y: usize) {
    let tile = board[x][y];
    let player2 = other_player(player);

    board[x][y] =
        match tile {
            Tile::Empty => Tile::Alive(player),
            Tile::Alive(player2) => Tile::Squashed(player),
            _ => { panic!("Invalid move") }
        }
}

pub fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    // TODO: use flat_map to find combinations
    // Can we return resulting iterator?
    let offs = [-1, 0, 1];

    let mut vec = Vec::new();
    vec.push((x, y));
    vec.iter().
    vec
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
        assert_eq!(Player::Red, other_player(Player::Blue));
        assert_eq!(Player::Blue, other_player(Player::Red));
    }

    #[test]
    #[should_panic]
    fn make_move_panics_on_invalid_move() {
        let mut board = create_board();
        let player = Player::Red;
        let (x, y) = base_pos(player);

        make_move(&mut board, player, x, y);
    }

    #[test]
    fn make_move_updates_empty_tile() {
        let mut board = create_board();
        let player = Player::Red;
        let (x, y) = base_pos(player);

        make_move(&mut board, player, x, y + 1);

        assert_eq!(Tile::Alive(player), board[x][y + 1]);
    }

    #[test]
    fn make_move_updates_alive_tile() {
        let mut board = create_board();
        let player = Player::Red;
        let player2 = Player::Blue;
        let (x, y) = base_pos(player);

        make_move(&mut board, player, x, y + 1);
        make_move(&mut board, player2, x, y + 1);

        assert_eq!(Tile::Squashed(player2), board[x][y + 1]);
    }
}