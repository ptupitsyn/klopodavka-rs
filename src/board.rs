use crate::models::*;

pub fn base_pos(p: Player) -> (usize, usize) {
    match p {
        Player::Red => (BOARD_WIDTH - BASE_OFFSET - 1, BASE_OFFSET),
        Player::Blue => (BASE_OFFSET, BOARD_HEIGHT - BASE_OFFSET - 1),
    }
}

fn place_base(p: Player, t: &mut Tiles) {
    let (x, y) = base_pos(p);
    t[x][y] = Tile::Base(p);
}

pub fn create_board() -> Tiles {
    let mut res = [[Tile::Empty; BOARD_WIDTH]; BOARD_HEIGHT];

    place_base(Player::Red, &mut res);
    place_base(Player::Blue, &mut res);

    res
}
