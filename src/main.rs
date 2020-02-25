mod board;
mod game;
mod models;

use board::*;
use game::*;
use models::*;

use rand::Rng;
use std::mem;

// TODO: Clippy
// TODO: std::iter::from_fn
fn main() {
    let game = create_game();
    let mut board = game.board;

    for i in 1..20 {
        let player = if i % 2 == 0 {
            Player::Red
        } else {
            Player::Blue
        };

        let all_moves = moves(&board, player);

        if all_moves.is_empty() {
            break;
        }

        let idx = rand::thread_rng().gen_range(0, all_moves.len());
        println!("{} of {}", idx, all_moves.len());
        let (x, y) = all_moves[idx];
        make_move(&mut board, player, x, y);
    }

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
