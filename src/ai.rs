use crate::game;

pub fn get_ai_move(game: &mut game::GameState) -> Option<(usize, usize)> {
    let moves = game.moves();

    if moves.is_empty() {
        return Option::None;
    }

    let tiles = game.board();

    let attack_move = moves
        .iter()
        .map(|&(x, y)| (tiles[x][y], (x, y)))
        .find(|&t| t.0.is_alive());

    match attack_move {
        None => {}
        Some(tile) => return Option::Some(tile.1),
    }

    //for (x, y) in moves {}

    Option::None
}
