use crate::board;
use crate::game::GameState;
use crate::models::{Pos, TilePos};
use rand::seq::IteratorRandom;
use std::i32;

pub fn moves(game: &GameState) -> impl Iterator<Item = TilePos> + '_ {
    game.moves().iter().map(move |&pos| TilePos {
        pos,
        tile: game.tile(pos),
    })
}

pub fn get_ai_move(game: &GameState) -> Option<TilePos> {
    if game.moves().is_empty() {
        return Option::None;
    }

    attack_move(game)
        .or_else(|| advance_move(game))
        .or_else(|| random_move(game))
}

fn attack_move(game: &GameState) -> Option<TilePos> {
    moves(game).find(|&t| t.tile.is_alive())
}

fn advance_move(game: &GameState) -> Option<TilePos> {
    let has_one_neighbor = |p: Pos| {
        board::neighbors(p)
            .iter()
            .filter(|&&n| !game.tile(n).is_empty())
            .count()
            == 1
    };

    moves(game).find(|&t| t.tile.is_empty() && has_one_neighbor(t.pos))
}

fn random_move(game: &GameState) -> Option<TilePos> {
    moves(game).choose(&mut rand::thread_rng())
}

fn get_advance_moves_with_weights(game: &GameState) -> Vec<(Pos, u32)> {
    // Compute move weights based on:
    // * Neighbor count - less is better
    // * Diagonal - true is better
    // * Enemy base distance - less is better

    let res: Vec<(Pos, u32)> = Vec::new();
    let enemy_base = board::base_pos(game.current_player().other());

    for mv in moves(game) {
        let mut weight = 1;

        let neighbs = board::neighbors(mv.pos);

        let nonempty_neighbs = neighbs
            .iter()
            .filter(|n| !game.tile(**n).is_empty())
            .count();

        let diag_neighbs = neighbs
            .iter()
            .filter(|n| n.x != mv.pos.x && n.y != mv.pos.y)
            .count();
    }

    res
}

/*fn dist(a: Pos, b: Pos) -> u32 {
    i32::ab(a.x - b.x)
}*/
