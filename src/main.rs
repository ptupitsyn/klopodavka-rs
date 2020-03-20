mod board;
mod game;
mod models;

use models::*;

use rand::Rng;
use std::mem;

// TODO: std::iter::from_fn
fn main() {
    let mut game = game::create_game();

    for _i in 1..20 {
        let all_moves = game::moves(&game);

        if all_moves.is_empty() {
            break;
        }

        let idx = rand::thread_rng().gen_range(0, all_moves.len());
        println!("{} of {}", idx, all_moves.len());
        let (x, y) = all_moves[idx];
        game::make_move(&mut game, x, y);
    }

    println!("Tile size: {}", mem::size_of::<Tile>());
    println!("Tiles size: {}", mem::size_of::<Tiles>());

    let board_str = print_board(&game.board);
    println!("{}", board_str);
}

fn print_board(board: &Tiles) -> String {
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

    #[allow(clippy::needless_range_loop)]
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
