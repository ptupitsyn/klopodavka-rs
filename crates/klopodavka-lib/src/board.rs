use crate::models::*;

pub type Board = Tiles<Tile>;

pub fn base_pos(p: Player, size: Size) -> Pos {
    match p {
        Player::Red => Pos {
            x: BASE_OFFSET,
            y: size.height - BASE_OFFSET - 1,
        },
        Player::Blue => Pos {
            x: size.width - BASE_OFFSET - 1,
            y: BASE_OFFSET,
        },
    }
}

pub fn pos_iter(size: Size) -> impl Iterator<Item = Pos> {
    (0..size.width).flat_map(move |x| (0..size.height).map(move |y| Pos { x, y }))
}

pub fn create_board() -> Board {
    create_board_with_size(Size {
        width: 30,
        height: 30,
    })
}

pub fn create_board_with_size(size: Size) -> Board {
    let mut res = Tiles::with_size(size);

    fn place_base(p: Player, t: &mut Board) {
        let pos = base_pos(p, t.size());
        t[pos] = Tile::Base(p);
    }

    place_base(Player::Red, &mut res);
    place_base(Player::Blue, &mut res);

    res
}

pub fn make_move(board: &mut Board, player: Player, x: Bsize, y: Bsize) {
    let player2 = player.other();

    let tile = match board.get(x, y) {
        Some(Tile::Empty) => Tile::Alive(player),
        Some(Tile::Alive(p)) if p == player2 => Tile::Squashed(player),
        other => panic!("Invalid move: from {:?} to {:?}", other, player),
    };

    board.set(x, y, tile);
}

pub fn neighbors(pos: Pos, size: Size) -> impl Iterator<Item = Pos> {
    neighbors_dist(pos, size, 1)
}

pub fn neighbors_dist(pos: Pos, size: Size, dist: u8) -> impl Iterator<Item = Pos> + 'static {
    let (w, h) = (size.width as i32, size.height as i32);
    let (x, y) = (pos.x as i32, pos.y as i32);
    let idist = dist as i32;
    let range = -idist..idist + 1;

    range
        .clone()
        .flat_map(move |a| range.clone().map(move |b| (a + x, b + y)))
        .filter(move |&(a, b)| a >= 0 && b >= 0 && a < w && b < h && (a, b) != (x, y))
        .map(|(a, b)| Pos {
            x: a as u16,
            y: b as u16,
        })
}

pub fn moves(board: &Board, player: Player) -> impl Iterator<Item = Pos> + '_ {
    connected_tiles(board, player, true)
}

pub fn connected_tiles(
    board: &Board,
    player: Player,
    return_potential_moves: bool,
) -> impl Iterator<Item = Pos> + '_ {
    let mut visited: Tiles<bool> = Tiles::with_size(board.size());
    let mut stack = Vec::new();
    stack.push(base_pos(player, board.size()));
    let enemy = player.other();

    // Traverse the tree of connected tiles and return all reachable empty tiles.
    std::iter::from_fn(move || {
        while let Some(pos) = stack.pop() {
            if !visited[pos] {
                visited[pos] = true;

                let tile = board[pos];

                if tile == Tile::Empty || tile == Tile::Alive(enemy) {
                    if return_potential_moves {
                        return Some(pos);
                    }
                } else if tile == Tile::Base(player)
                    || tile == Tile::Squashed(player)
                    || tile == Tile::Alive(player)
                {
                    for neighbor in neighbors(pos, board.size()) {
                        stack.push(neighbor);
                    }

                    if !return_potential_moves {
                        return Some(pos);
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
    use crate::models::{Player, Pos, Tile};

    #[test]
    fn create_board_returns_empty_board_with_bases() {
        let board = create_board();
        let base_red = base_pos(Player::Red, board.size());
        let base_blue = base_pos(Player::Blue, board.size());

        for x in 0..board.size().width {
            for y in 0..board.size().height {
                let pos = Pos { x, y };
                let tile = board[pos];
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
        let pos = base_pos(player, board.size());
        make_move(&mut board, player, pos.x, pos.y);
    }

    #[test]
    fn make_move_updates_empty_tile() {
        let mut board = create_board();
        let player = Player::Red;
        let pos = base_pos(player, board.size());

        make_move(&mut board, player, pos.x, pos.y + 1);

        assert_eq!(board.get(pos.x, pos.y + 1), Some(Tile::Alive(player)));
    }

    #[test]
    fn make_move_updates_alive_tile() {
        let mut board = create_board();
        let player = Player::Red;
        let player2 = Player::Blue;
        let base = base_pos(player, board.size());
        let pos = Pos {
            x: base.x,
            y: base.y + 1,
        };

        make_move(&mut board, player, pos.x, pos.y);
        make_move(&mut board, player2, pos.x, pos.y);

        assert_eq!(board.getp(pos), Some(Tile::Squashed(player2)));
    }

    #[test]
    fn neighbors_returns_8_tiles_for_mid_board() {
        let x = 3;
        let y = 5;
        let res: Vec<Pos> = neighbors(Pos { x, y }, create_board().size()).collect();

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
        let res: Vec<Pos> = neighbors(Pos { x: 0, y: 0 }, create_board().size()).collect();

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

        let base = base_pos(Player::Red, board.size());
        let mut expected: Vec<Pos> = neighbors(base, board.size()).collect();
        expected.reverse();

        println!("{:?}", res);
        assert_eq!(res, expected);
    }

    #[test]
    fn moves_returns_neighbors_of_all_live_tiles() {
        let mut board = create_board();
        let base = base_pos(Player::Red, board.size());
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
