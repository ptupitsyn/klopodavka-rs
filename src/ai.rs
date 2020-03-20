use crate::game;
use crate::models::{Pos, TilePos};

pub fn get_ai_move(game: &mut game::GameState) -> Option<TilePos> {
    if game.moves().is_empty() {
        return Option::None;
    }

    let attack_move = game.moves2().find(|&t| t.tile.is_alive());

    if let Some(res) = attack_move {
        return attack_move;
    }

    //for (x, y) in moves {}

    Option::None
}
