mod utils;

use wasm_bindgen::prelude::*;

use klopodavka_lib::{ai, game};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let mut game = game::GameState::new();

    for _i in 1..20 {
        match ai::get_ai_move(&game) {
            Some(pos) => game.make_move(pos),
            None => break,
        }
    }

    let board_str = format!("Hello from klopodavka: {}", game);

    alert(&board_str);
}
