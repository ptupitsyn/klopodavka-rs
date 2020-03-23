use klopodavka_lib::{ai, game};

fn main() {
    let mut game = game::GameState::new();
    let mut move_count = 0;

    for _i in 1.. {
        move_count += 1;

        match ai::get_ai_move(&game) {
            Some(tile) => game.make_move(tile.pos),
            None => break,
        }
    }
    println!("Total moves: {}", move_count);
    println!("{}", game);
}
