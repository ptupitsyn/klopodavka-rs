mod ai;
mod board;
mod game;
mod models;

use models::*;

use std::mem;

// TODO: std::iter::from_fn
// TODO: Encapsulate game stuff in a library, create different frontends (console, web, ...)
fn main() {
    let mut game = game::GameState::new();

    for _i in 1..20 {
        match ai::get_ai_move(&mut game) {
            Some(tile) => game.make_move(tile.pos),
            None => break,
        }
    }

    println!("Tile size: {}", mem::size_of::<Tile>());
    println!("Tiles size: {}", mem::size_of::<Tiles>());

    println!("{}", game);
}
