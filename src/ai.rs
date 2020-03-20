use crate::game;
use crate::game::GameState;
use crate::models::TilePos;

pub fn get_ai_move(game: &mut game::GameState) -> Option<TilePos> {
    if game.moves().is_empty() {
        return Option::None;
    }

    attack_move(game).or_else(|| advance_move(game))
}

fn attack_move(game: &mut GameState) -> Option<TilePos> {
    game.moves2().find(|&t| t.tile.is_alive())
}

fn advance_move(game: &mut GameState) -> Option<TilePos> {
    // TODO: Pick diagonal move closest to enemy base
    game.moves2().find(|&t| !t.tile.is_alive())
}
