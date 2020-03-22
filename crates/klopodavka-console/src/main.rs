use klopodavka_lib::{ai, game};

fn main() {
    let mut game = game::GameState::new();

    for _i in 1..20 {
        match ai::get_ai_move(&game) {
            Some(tile) => game.make_move(tile.pos),
            None => break,
        }
    }

    println!("{}", game);
}
