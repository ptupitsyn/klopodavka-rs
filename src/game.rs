use crate::board;
use crate::models::*;

pub fn create_game() -> GameState {
    GameState {
        board: board::create_board(),
        current_player: Player::Red,
        moves_left: 5,
        turn_length: 5,
    }
}

pub fn make_move(game: &mut GameState, x: usize, y: usize) {
    crate::board::make_move(&mut game.board, game.current_player, x, y);

    let last = game.moves_left == 1;
    let left = if last {
        game.turn_length
    } else {
        game.moves_left - 1
    };
    game.moves_left = left;

    if last {
        game.current_player = board::other_player(game.current_player);
    }
}

#[cfg(test)]
mod tests {
    use crate::board;
    use crate::game::*;
    use crate::models::{Player, Tile};

    #[test]
    fn create_game_returns_new_game_state() {
        let game = create_game();

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
    }

    #[test]
    #[should_panic]
    fn make_move_panics_on_invalid_move() {
        let mut game = create_game();
        let (bx, by) = board::base_pos(game.current_player);

        make_move(&mut game, bx, by);
    }

    #[test]
    fn make_move_updates_board_and_move_count() {
        let game = create_game();
    }
}
