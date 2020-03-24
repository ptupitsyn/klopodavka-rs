use klopodavka_lib::{ai, game};

fn main() {
    // This console app is mainly for perf profiling:
    // cargo flamegraph --bin klopodavka-console

    let mut move_count = 0;

    for _j in 1..10000 {
        let mut game = game::GameState::new();
        game.disable_heat_map = true; // Affects performance a lot.

        for _i in 1.. {
            move_count += 1;

            match ai::get_ai_move(&game) {
                Some(tile) => game.make_move(tile.pos),
                None => break,
            }
        }
    }

    println!("Total moves: {}", move_count);
}
