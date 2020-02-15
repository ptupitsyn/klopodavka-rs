mod board;
mod models;

use crate::board::*;

fn main() {
    let board = create_board();
    println!("Board: {:#?}", board);
}
