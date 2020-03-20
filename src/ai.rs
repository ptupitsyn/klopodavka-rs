use crate::game;
use crate::game::GameState;
use crate::models::TilePos;
use rand::seq::IteratorRandom;

pub fn get_ai_move(game: &mut game::GameState) -> Option<TilePos> {
    if game.moves().is_empty() {
        return Option::None;
    }

    attack_move(game)
        .or_else(|| advance_move(game))
        .or_else(|| random_move(game))
}

fn attack_move(game: &mut GameState) -> Option<TilePos> {
    game.moves2().find(|&t| t.tile.is_alive())
}

fn advance_move(game: &mut GameState) -> Option<TilePos> {
    // TODO: Pick diagonal move closest to enemy base
    game.moves2().find(|&t| t.tile.is_empty())
}

fn random_move(game: &mut GameState) -> Option<TilePos> {
    game.moves2().choose(&mut rand::thread_rng())
}
