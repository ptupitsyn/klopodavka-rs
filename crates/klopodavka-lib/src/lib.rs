pub mod ai;
pub mod board;
pub mod game;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::models::{Tile, Tiles};
    use crate::{ai, game};
    use std::mem;

    #[test]
    fn it_works() {
        let mut game = game::GameState::new();

        for _i in 1..20 {
            match ai::get_ai_move(&game) {
                Some(pos) => game.make_move(pos),
                None => break,
            }
        }

        println!("Tile size: {}", mem::size_of::<Tile>());
        println!("Tiles size: {}", mem::size_of::<Tiles>());

        println!("{}", game);
    }
}
