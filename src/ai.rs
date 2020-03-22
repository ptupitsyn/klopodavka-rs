use crate::board;
use crate::game::GameState;
use crate::models::{Pos, TilePos};
use rand::seq::IteratorRandom;

pub fn moves(game: &GameState) -> impl Iterator<Item = TilePos> + '_ {
    game.moves().iter().map(move |&pos| TilePos {
        pos,
        tile: game.board()[pos.x][pos.y],
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
    // TODO: Compute move weights based on:
    // * Neighbor count - less is better
    // * Diagonal - true is better
    // * Enemy base distance - less is better

    let board = game.board();

    let has_one_neighbor = |p: Pos| {
        board::neighbors(p)
            .iter()
            .filter(|n| !board[n.x][n.y].is_empty())
            .count()
            == 1
    };

    moves(game).find(|&t| t.tile.is_empty() && has_one_neighbor(t.pos))
}

fn random_move(game: &GameState) -> Option<TilePos> {
    moves(game).choose(&mut rand::thread_rng())
}

//fn get_tile_weighs(gam)
