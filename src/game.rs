use crate::board::*;
use crate::models::*;

pub fn create_game() -> GameState {
    GameState {
        board: create_board(),
        current_player: Player::Red,
        moves_left: 5,
        turn_length: 5,
    }
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::game::*;
    use crate::models::{Player, Tile, BOARD_HEIGHT, BOARD_WIDTH};

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
}
