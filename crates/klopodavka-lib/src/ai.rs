use crate::board;
use crate::game::GameState;
use crate::models::{Pos, TilePos};
use rand::seq::IteratorRandom;
use std::i16;

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
    moves(game).min_by(|&x, &y| weight(game, x.pos).cmp(&weight(game, y.pos)))
}

fn random_move(game: &GameState) -> Option<TilePos> {
    moves(game).choose(&mut rand::thread_rng())
}

fn weight(game: &GameState, pos: Pos) -> u16 {
    // Compute move weights based on:
    // * Neighbor count - less is better
    // * Diagonal - true is better
    // * Enemy base distance - less is better

    let neighbs = board::neighbors(pos);

    let nonempty_neighbs = neighbs
        .iter()
        .filter(|n| !game.tile(**n).is_empty())
        .count();

    let diag_neighbs = neighbs
        .iter()
        .filter(|n| n.x != pos.x && n.y != pos.y && !game.tile(**n).is_empty())
        .count();

    let nondiag_neighbs = nonempty_neighbs - diag_neighbs;

    // TODO: Base positions should be saved within GameState
    let enemy_base = board::base_pos(game.current_player().other());
    let base_dist = dist(enemy_base, pos);

    let weight = diag_neighbs + nondiag_neighbs * 2 + base_dist as usize;

    weight as u16
}

fn dist(a: Pos, b: Pos) -> u16 {
    let dx = (a.x as i16 - b.x as i16).abs();
    let dy = (a.y as i16 - b.y as i16).abs();

    (dx + dy) as u16
}