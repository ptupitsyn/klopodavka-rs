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
        Tile::Alive(p) if p == player2 => Tile::Squashed(player),
        _ => panic!("Invalid move"),
    }
}

pub fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let (w, h) = (BOARD_WIDTH as i32, BOARD_HEIGHT as i32);
    let (_x, _y) = (x as i32, y as i32);

    let offs: [i32; 3] = [-1, 0, 1];

    offs.iter()
        .flat_map(|&a| offs.iter().map(move |&b| (a + _x, b + _y)))
        .filter(|&(a, b)| a >= 0 && b >= 0 && a < w && b < h && (a, b) != (_x, _y))
        .map(|(a, b)| (a as usize, b as usize))
        .collect()
}

pub fn moves(board: &Tiles, player: Player) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let mut visited = [[false; BOARD_HEIGHT]; BOARD_WIDTH];
    let mut stack = Vec::new();
    stack.push(base_pos(player));
    let enemy = other_player(player);

    loop {
        match stack.pop() {
            None => break,
            Some((x, y)) => {
                if !(visited[x][y]) {
                    visited[x][y] = true;

                    let tile = board[x][y];

                    if tile == Tile::Empty || tile == Tile::Alive(enemy) {
                        res.push((x, y));
                    } else if tile == Tile::Base(player)
                        || tile == Tile::Squashed(player)
                        || tile == Tile::Alive(player)
                    {
                        for (nx, ny) in neighbors(x, y) {
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::models::{Player, Tile, BOARD_HEIGHT, BOARD_WIDTH};

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

        for a in x - 1..x + 2 {
            for b in y - 1..y + 2 {
                let is_mid = a == x && b == y;
                assert_eq!(!is_mid, res.contains(&(a, b)));
            }
        }
    }

    #[test]
    fn neighbors_returns_3_tiles_for_corner() {
        let res = neighbors(0, 0);

        println!("{:?}", res);
        assert_eq!(res.len(), 3);

        for a in 0..2 {
            for b in 0..2 {
                let is_mid = a == 0 && b == 0;
                assert_eq!(!is_mid, res.contains(&(a, b)));
            }
        }
    }

    #[test]
    fn moves_returns_base_neighbors_for_new_board() {
        let board = create_board();
        let res = moves(&board, Player::Red);

        let (bx, by) = base_pos(Player::Red);
        let mut expected = neighbors(bx, by);
        expected.reverse();

        println!("{:?}", res);
        assert_eq!(res, expected);
    }

    #[test]
    fn moves_returns_neighbors_of_all_live_tiles() {
        let mut board = create_board();
        let (bx, by) = base_pos(Player::Red);
        make_move(&mut board, Player::Red, bx + 1, by - 1);
        make_move(&mut board, Player::Red, bx + 2, by - 2);

        let res = moves(&board, Player::Red);

        assert!(res.contains(&(bx + 3, by - 3)));
        assert!(res.contains(&(bx + 2, by - 3)));
        assert!(res.contains(&(bx + 1, by - 3)));
    }
}
