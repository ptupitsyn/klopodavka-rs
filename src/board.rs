use crate::models::*;

pub fn base_pos(p: Player) -> (usize, usize) {
    match p {
        Player::Red => (BASE_OFFSET, BOARD_HEIGHT - BASE_OFFSET - 1),
        Player::Blue => (BOARD_WIDTH - BASE_OFFSET - 1, BASE_OFFSET),
    }
}

fn place_base(p: Player, t: &mut Tiles) {
    let (x, y) = base_pos(p);
    t[x][y] = Tile::Base(p);
}

pub fn create_board() -> Tiles {
    let mut res = [[Tile::Empty; BOARD_HEIGHT]; BOARD_WIDTH];

    place_base(Player::Red, &mut res);
    place_base(Player::Blue, &mut res);

    res
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
                }
                else {
                    assert_eq!(Tile::Empty, tile);
                }
            }
        }
    }
}