use crate::board;
use crate::models::*;

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: board::create_board(),
            current_player: Player::Red,
            moves_left: 5,
            turn_length: 5,
        }
    }

    pub fn moves(self: &Self) -> Vec<(usize, usize)> {
        board::moves(&self.board, self.current_player)
    }

    pub fn make_move(self: &mut Self, x: usize, y: usize) {
        crate::board::make_move(&mut self.board, self.current_player, x, y);

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
    }
}

#[cfg(test)]
mod tests {
    use crate::board;
    use crate::models::Tile::Alive;
    use crate::models::{GameState, Player, Tile};
    use rand::Rng;

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
    fn make_move_panics_on_invalid_move() {
        let mut game = GameState::new();
        let (bx, by) = board::base_pos(game.current_player);

        game.make_move(bx, by);
    }

    #[test]
    fn make_move_updates_board_and_move_count() {
        let mut game = GameState::new();
        let (bx, by) = board::base_pos(game.current_player);
        game.make_move(bx, by + 1);

        assert_eq!(game.moves_left, game.turn_length - 1);
        assert_eq!(game.board[bx][by + 1], Alive(game.current_player));
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
            let (x, y) = all_moves[idx];

            game.make_move(x, y);
        }

        println!("{}", game);
    }
}
