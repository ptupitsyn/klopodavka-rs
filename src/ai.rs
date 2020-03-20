use crate::board;
use crate::game;
use crate::game::GameState;
use crate::models::{Pos, TilePos};
use rand::seq::IteratorRandom;

pub fn moves(game: &GameState) -> impl Iterator<Item = TilePos> + '_ {
    game.moves().iter().map(move |&pos| TilePos {
        pos,
        tile: game.board()[pos.x][pos.y],
    })
}

pub fn get_ai_move(game: &mut game::GameState) -> Option<TilePos> {
    if game.moves().is_empty() {
        return Option::None;
    }

    attack_move(game)
        .or_else(|| advance_move(game))
        .or_else(|| random_move(game))
}

fn attack_move(game: &mut GameState) -> Option<TilePos> {
    moves(game).find(|&t| t.tile.is_alive())
}

fn advance_move(game: &mut GameState) -> Option<TilePos> {
    // TODO: Pick diagonal move closest to enemy base
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

fn random_move(game: &mut GameState) -> Option<TilePos> {
    moves(game).choose(&mut rand::thread_rng())
}
