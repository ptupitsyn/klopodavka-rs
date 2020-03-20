mod ai;
mod board;
mod game;
mod models;

use models::*;

use rand::Rng;
use std::mem;

// TODO: std::iter::from_fn
// TODO: Encapsulate game stuff in a library, create different frontends (console, web, ...)
fn main() {
    let mut game = game::GameState::new();

    for _i in 1..20 {
        let all_moves = game.moves();

        if all_moves.is_empty() {
            break;
        }

        let idx = rand::thread_rng().gen_range(0, all_moves.len());
        println!(
            "{:?}: {} of {}",
            game.current_player(),
            idx,
            all_moves.len()
        );
        let (x, y) = all_moves[idx];
        game.make_move(x, y);
    }

    println!("Tile size: {}", mem::size_of::<Tile>());
    println!("Tiles size: {}", mem::size_of::<Tiles>());

    println!("{}", game);
}
