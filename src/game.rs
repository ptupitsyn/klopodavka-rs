use crate::board::*;
use crate::models::*;

pub fn create_game() -> GameState {
    let g = GameState {
        board: create_board(),
        current_player: Player::Red,
        moves_left: 5,
        turn_length: 5,
    };

    g
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::game::*;
    use crate::models::{Player, Tile, BOARD_HEIGHT, BOARD_WIDTH};

    #[test]
    fn create_game_returns_new_game_state() {
        let game = create_game();

        assert_eq!(game.current_player, Player::Red);
    }
}
