use crate::board;
use crate::models::*;
use std::fmt;

pub struct GameState {
    board: Tiles,
    current_player: Player,
    turn_length: u32,
    moves_left: u32,
    moves: Vec<Pos>,
}

impl GameState {
    pub fn new() -> GameState {
        let tiles = board::create_board();
        let player = Player::Red;

        GameState {
            board: tiles,
            current_player: player,
            moves_left: 5,
            turn_length: 5,
            moves: board::moves(&tiles, player),
        }
    }

    pub fn moves(&self) -> &Vec<Pos> {
        self.moves.as_ref()
    }

    pub fn moves2(&self) -> impl Iterator<Item = TilePos> + '_ {
        self.moves.iter().map(move |&pos| TilePos {
            pos,
            tile: self.board[pos.x][pos.y],
        })
    }

    pub fn make_move(&mut self, pos: Pos) {
        let valid = self.moves.contains(&pos);
        if !valid {
            panic!("Invalid move: {:?}", pos)
        }

        crate::board::make_move(&mut self.board, self.current_player, pos.x, pos.y);

        let last = self.moves_left == 1;

        let left = if last {
            self.turn_length
        } else {
            self.moves_left - 1
        };

        self.moves_left = left;

        if last {
            self.current_player = board::other_player(self.current_player);
        }

        self.moves = board::moves(&self.board, self.current_player);
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                let tile = self.board[x][y];
                let ch = get_ch(tile);
                res.push(ch);
            }
            res.push('\n');
        }

        write!(
            f,
            "{:?}, {} of {}\n{}",
            self.current_player, self.moves_left, self.turn_length, res
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::board;
    use crate::game::GameState;
    use crate::models::Tile::Alive;
    use crate::models::{Player, Pos, Tile};
    use rand::Rng;

    impl Pos {
        pub fn new(x: usize, y: usize) -> Pos {
            Pos { x, y }
        }
    }

    #[test]
    fn create_game_returns_new_game_state() {
        let game = GameState::new();

        let tiles = game.board.iter().flat_map(|r| r.iter());
        for tile in tiles {
            match tile {
                Tile::Empty => {}
                Tile::Base(_) => {}
                Tile::Alive(_) => panic!("Alive tile on new board"),
                Tile::Squashed(_) => panic!("Squashed tile on new board"),
            }
        }

        assert_eq!(game.current_player, Player::Red);
        assert_eq!(game.turn_length, 5);
        assert_eq!(game.moves_left, 5);

        println!("{}", game);
    }

    #[test]
    #[should_panic]
    fn make_move_panics_on_invalid_move_to_base() {
        let mut game = GameState::new();
        let pos = board::base_pos(game.current_player);

        game.make_move(pos);
    }

    #[test]
    #[should_panic]
    fn make_move_panics_on_invalid_move_to_disconnected_tile() {
        let mut game = GameState::new();

        game.make_move(Pos::new(0, 0));
    }

    #[test]
    fn make_move_updates_board_and_move_count() {
        let mut game = GameState::new();
        let base_pos = board::base_pos(game.current_player);
        let pos = Pos::new(base_pos.x, base_pos.y + 1);
        game.make_move(pos);

        assert_eq!(game.moves_left, game.turn_length - 1);
        assert_eq!(game.board[pos.x][pos.y], Alive(game.current_player));
    }

    #[test]
    fn make_random_move_fills_board_until_finished() {
        let mut game = GameState::new();

        loop {
            let all_moves = game.moves();

            if all_moves.is_empty() {
                break;
            }

            let idx = rand::thread_rng().gen_range(0, all_moves.len());

            game.make_move(all_moves[idx]);
        }

        println!("{}", game);
    }
}
