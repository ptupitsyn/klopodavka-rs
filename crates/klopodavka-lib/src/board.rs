use crate::models::*;

pub fn base_pos(p: Player) -> Pos {
    match p {
        Player::Red => Pos {
            x: BASE_OFFSET,
            y: BOARD_HEIGHT - BASE_OFFSET - 1,
        },
        Player::Blue => Pos {
            x: BOARD_WIDTH - BASE_OFFSET - 1,
            y: BASE_OFFSET,
        },
    }
}

pub fn create_board() -> Tiles {
    let mut res = [[Tile::Empty; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

    fn place_base(p: Player, t: &mut Tiles) {
        let pos = base_pos(p);
        t[pos.x as usize][pos.y as usize] = Tile::Base(p);
    }

    place_base(Player::Red, &mut res);
    place_base(Player::Blue, &mut res);

    res
}

pub fn make_move(board: &mut Tiles, player: Player, x: u16, y: u16) {
    let player2 = player.other();
    let (xx, yy) = (x as usize, y as usize);

    board[xx][yy] = match board[xx][yy] {
        Tile::Empty => Tile::Alive(player),
        Tile::Alive(p) if p == player2 => Tile::Squashed(player),
        other => panic!("Invalid move: from {:?} to {:?}", other, player),
    }
}

pub fn neighbors(pos: Pos) -> impl Iterator<Item = Pos> {
    neighbors_dist(pos, 1)
}

pub fn neighbors_dist(pos: Pos, dist: u8) -> impl Iterator<Item = Pos> + 'static {
    let (w, h) = (BOARD_WIDTH as i32, BOARD_HEIGHT as i32);
    let (_x, _y) = (pos.x as i32, pos.y as i32);
    let n = dist as i32;
    let range = -n..n + 1;

    range
        .clone()
        .flat_map(move |a| range.clone().map(move |b| (a + _x, b + _y)))
        .filter(move |&(a, b)| a >= 0 && b >= 0 && a < w && b < h && (a, b) != (_x, _y))
        .map(|(a, b)| Pos {
            x: a as u16,
            y: b as u16,
        })
}

pub fn moves(board: &Tiles, player: Player) -> impl Iterator<Item = Pos> + '_ {
    let mut visited = [[false; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];
    let mut stack = Vec::new();
    stack.push(base_pos(player));
    let enemy = player.other();

    // Traverse the tree of connected tiles and return all reachable empty tiles.
    std::iter::from_fn(move || {
        while let Some(pos) = stack.pop() {
            let (x, y) = (pos.x as usize, pos.y as usize);

            if !(visited[x][y]) {
                visited[x][y] = true;

                let tile = board[x][y];

                if tile == Tile::Empty || tile == Tile::Alive(enemy) {
                    return Some(pos);
                } else if tile == Tile::Base(player)
                    || tile == Tile::Squashed(player)
                    || tile == Tile::Alive(player)
                {
                    for neighbor in neighbors(pos) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        None
    })
}

pub fn dist(a: Pos, b: Pos) -> u16 {
    let dx = (a.x as i16 - b.x as i16).abs();
    let dy = (a.y as i16 - b.y as i16).abs();

    // Because diagonal moves are allowed, distance is max of two.
    std::cmp::max(dx, dy) as u16
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::models::{Player, Pos, Tile, BOARD_HEIGHT, BOARD_WIDTH};

    #[test]
    fn create_board_returns_empty_board_with_bases() {
        let board = create_board();
        let base_red = base_pos(Player::Red);
        let base_blue = base_pos(Player::Blue);

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let tile = board[x as usize][y as usize];
                let pos = Pos { x, y };
                if pos == base_red {
                    assert_eq!(tile, Tile::Base(Player::Red));
                } else if pos == base_blue {
                    assert_eq!(tile, Tile::Base(Player::Blue));
                } else {
                    assert_eq!(tile, Tile::Empty);
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn make_move_panics_on_invalid_move() {
        let mut board = create_board();
        let player = Player::Red;
        let pos = base_pos(player);
        make_move(&mut board, player, pos.x, pos.y);
    }

    #[test]
    fn make_move_updates_empty_tile() {
        let mut board = create_board();
        let player = Player::Red;
        let pos = base_pos(player);

        make_move(&mut board, player, pos.x, pos.y + 1);

        assert_eq!(
            board[pos.x as usize][(pos.y + 1) as usize],
            Tile::Alive(player)
        );
    }

    #[test]
    fn make_move_updates_alive_tile() {
        let mut board = create_board();
        let player = Player::Red;
        let player2 = Player::Blue;
        let base = base_pos(player);
        let pos = Pos {
            x: base.x,
            y: base.y + 1,
        };

        make_move(&mut board, player, pos.x, pos.y);
        make_move(&mut board, player2, pos.x, pos.y);

        assert_eq!(
            board[pos.x as usize][pos.y as usize],
            Tile::Squashed(player2)
        );
    }

    #[test]
    fn neighbors_returns_8_tiles_for_mid_board() {
        let x = 3;
        let y = 5;
        let res: Vec<Pos> = neighbors(Pos { x, y }).collect();

        println!("{:?}", res);
        assert_eq!(res.len(), 8);

        for a in x - 1..x + 2 {
            for b in y - 1..y + 2 {
                let is_mid = a == x && b == y;
                assert_eq!(!is_mid, res.contains(&Pos { x: a, y: b }));
            }
        }
    }

    #[test]
    fn neighbors_returns_3_tiles_for_corner() {
        let res: Vec<Pos> = neighbors(Pos { x: 0, y: 0 }).collect();

        println!("{:?}", res);
        assert_eq!(res.len(), 3);

        for a in 0..2 {
            for b in 0..2 {
                let is_mid = a == 0 && b == 0;
                assert_eq!(!is_mid, res.contains(&Pos { x: a, y: b }));
            }
        }
    }

    #[test]
    fn moves_returns_base_neighbors_for_new_board() {
        let board = create_board();
        let res: Vec<Pos> = moves(&board, Player::Red).collect();

        let base = base_pos(Player::Red);
        let mut expected: Vec<Pos> = neighbors(base).collect();
        expected.reverse();

        println!("{:?}", res);
        assert_eq!(res, expected);
    }

    #[test]
    fn moves_returns_neighbors_of_all_live_tiles() {
        let mut board = create_board();
        let base = base_pos(Player::Red);
        let (bx, by) = (base.x, base.y);
        make_move(&mut board, Player::Red, bx + 1, by - 1);
        make_move(&mut board, Player::Red, bx + 2, by - 2);

        let res: Vec<Pos> = moves(&board, Player::Red).collect();

        assert!(res.contains(&Pos::new(bx + 3, by - 3)));
        assert!(res.contains(&Pos::new(bx + 2, by - 3)));
        assert!(res.contains(&Pos::new(bx + 1, by - 3)));
    }

    #[test]
    fn dist_returns_number_of_moves_to_connect_tiles() {
        assert_eq!(dist(Pos { x: 1, y: 1 }, Pos { x: 1, y: 1 }), 0);
        assert_eq!(dist(Pos { x: 1, y: 1 }, Pos { x: 1, y: 2 }), 1);
        assert_eq!(dist(Pos { x: 1, y: 1 }, Pos { x: 2, y: 2 }), 1);
        assert_eq!(dist(Pos { x: 1, y: 1 }, Pos { x: 2, y: 1 }), 1);
    }
}
