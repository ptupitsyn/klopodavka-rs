mod board;
mod models;

use crate::board::*;
use crate::models::*;

use std::mem;

fn main() {
    let mut board = create_board();

    let (bx, by) = base_pos(Player::Red);
    make_move(&mut board, Player::Red, bx + 1, by + 1);

    println!("Tile size: {}", mem::size_of::<Tile>());
    println!("Tiles size: {}", mem::size_of::<Tiles>());

    let board_str = print_board(board);
    println!("{}", board_str);
}

fn print_board(board: Tiles) -> String {
    fn get_ch(tile: Tile) -> char {
        match tile {
            Tile::Empty => '.',
            Tile::Base(Player::Blue) => '⬛',
            Tile::Base(Player::Red) => '⬤',
            Tile::Alive(Player::Blue) => '◻',
            Tile::Alive(Player::Red) => '○',
            Tile::Squashed(Player::Blue) => '◼',
            Tile::Squashed(Player::Red) => '●',
        }
    }

    let mut res = String::new();

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let tile = board[x][y];
            let ch = get_ch(tile);
            res.push(ch);
        }
        res.push('\n');
    }

    res
}
