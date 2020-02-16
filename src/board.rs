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

    board[x][y] = match tile {
        Tile::Empty => Tile::Alive(player),
        Tile::Alive(player2) => Tile::Squashed(player),
        _ => panic!("Invalid move"),
    }
}

pub fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    let (w, h) = (BOARD_WIDTH as i32, BOARD_HEIGHT as i32);

    let offs: [i32; 3] = [-1, 0, 1];

    let pairs = offs
        .iter()
        .flat_map(|&a| offs.iter().map(move |&b| (a + x, b + y)))
        .filter(|&(a, b)| a >= 0 && b >= 0 && a < w && b < h && (a, b) != (x, y))
        .collect();

    pairs
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
                    assert_eq!(tile, Tile::Base(Player::Red));
                } else if x == bx && y == by {
                    assert_eq!(tile, Tile::Base(Player::Blue));
                } else {
                    assert_eq!(tile, Tile::Empty);
                }
            }
        }
    }

    #[test]
    fn get_other_player_returns_opponent() {
        assert_eq!(other_player(Player::Blue), Player::Red);
        assert_eq!(other_player(Player::Red), Player::Blue);
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

        assert_eq!(board[x][y + 1], Tile::Alive(player));
    }

    #[test]
    fn make_move_updates_alive_tile() {
        let mut board = create_board();
        let player = Player::Red;
        let player2 = Player::Blue;
        let (x, y) = base_pos(player);

        make_move(&mut board, player, x, y + 1);
        make_move(&mut board, player2, x, y + 1);

        assert_eq!(board[x][y + 1], Tile::Squashed(player2));
    }

    #[test]
    fn neighbors_returns_8_tiles_for_mid_board() {
        let x = 3;
        let y = 5;
        let res = neighbors(x, y);

        println!("{:?}", res);
        assert_eq!(res.len(), 8);

        for a in x - 1..x + 1 {
            for b in y - 1..y + 1 {
                let is_mid = a == x && b == y;
                assert_eq!(!is_mid, res.contains(&(a, b)));
            }
        }
    }
}
