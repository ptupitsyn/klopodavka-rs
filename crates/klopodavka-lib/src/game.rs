use crate::board;
use crate::models::*;
use std::fmt;

pub const TURN_LENGTH: u8 = 6;

pub type BoolTiles = [[bool; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HeatMapTile {
    pub red: u8,
    pub blue: u8,
}

type HeatMapTiles = [[HeatMapTile; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

pub struct GameState {
    board: Tiles,
    current_player: Player,
    turn_length: u32,
    moves_left: u32,
    moves: Vec<Pos>,
    moves_map: BoolTiles,
    heat_map: HeatMapTiles,
    pub disable_heat_map: bool,
}

fn update_moves(game: &mut GameState) {
    // Reuse existing vector.
    game.moves
        .splice(0.., board::moves(&game.board, game.current_player));

    // Reallocate map - faster than clean/update.
    game.moves_map = [[false; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

    for pos in game.moves.iter() {
        game.moves_map[pos.x as usize][pos.y as usize] = true;
    }
}

fn update_heat_map_incrementally(map: &mut HeatMapTiles, pos: Pos, player: Player) {
    let (x, y) = (pos.x as usize, pos.y as usize);

    let old = map[x][y];
    let max_val = TURN_LENGTH;

    map[x][y] = match player {
        Player::Red => HeatMapTile {
            red: max_val + 1,
            blue: old.blue,
        },
        Player::Blue => HeatMapTile {
            red: old.red,
            blue: max_val + 1,
        },
    };

    for pos2 in board::neighbors_dist(pos, max_val) {
        let (x, y) = (pos2.x as usize, pos2.y as usize);
        let old = map[x][y];
        let heat = max_val + 1 - board::dist(pos, pos2) as u8;

        map[x][y] = match player {
            Player::Red => HeatMapTile {
                blue: old.blue,
                red: std::cmp::max(old.red, heat),
            },
            Player::Blue => HeatMapTile {
                blue: std::cmp::max(old.blue, heat),
                red: old.red,
            },
        };
    }
}

fn update_heat_map_fully(game: &mut GameState, player: Player) {
    let mut map = &mut game.heat_map;

    for (x, y) in board::pos_iter() {
        match player {
            Player::Red => map[x][y].red = 0,
            Player::Blue => map[x][y].blue = 0,
        };
    }

    for connected_tile_pos in board::connected_tiles(&game.board, player, false) {
        update_heat_map_incrementally(map, connected_tile_pos, player);
    }
}

fn new_heat_map() -> HeatMapTiles {
    let mut res = [[HeatMapTile { blue: 0, red: 0 }; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

    update_heat_map_incrementally(&mut res, board::base_pos(Player::Red), Player::Red);
    update_heat_map_incrementally(&mut res, board::base_pos(Player::Blue), Player::Blue);

    res
}

fn new_moves_map() -> BoolTiles {
    [[false; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize]
}

#[allow(clippy::new_without_default)]
impl GameState {
    pub fn new() -> Self {
        GameState::new_custom(TURN_LENGTH as u32)
    }

    pub fn new_custom(turn_length: u32) -> Self {
        let tiles = board::create_board();
        let player = Player::Red;

        let mut res = GameState {
            board: tiles,
            current_player: player,
            turn_length,
            moves_left: turn_length,
            moves_map: new_moves_map(),
            moves: Vec::with_capacity(64),
            heat_map: new_heat_map(),
            disable_heat_map: false,
        };

        update_moves(&mut res);

        res
    }

    pub fn tile(&self, pos: Pos) -> Tile {
        self.board[pos.x as usize][pos.y as usize]
    }

    pub fn heat(&self, pos: Pos) -> HeatMapTile {
        self.heat_map[pos.x as usize][pos.y as usize]
    }

    pub fn max_heat(&self) -> u8 {
        self.turn_length as u8
    }

    pub fn tiles(&self) -> impl Iterator<Item = TilePos> + '_ {
        board::pos_iter()
            .map(|(x, y)| Pos {
                x: x as u16,
                y: y as u16,
            })
            .map(move |pos| TilePos {
                pos,
                tile: self.tile(pos),
            })
    }

    pub fn moves(&self) -> &Vec<Pos> {
        self.moves.as_ref()
    }

    pub fn is_valid_move(&self, pos: Pos) -> bool {
        self.moves_map[pos.x as usize][pos.y as usize]
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn current_base(&self) -> Pos {
        board::base_pos(self.current_player)
    }

    pub fn enemy_base(&self) -> Pos {
        board::base_pos(self.current_player.other())
    }

    pub fn winner(&self) -> Option<Player> {
        if self.moves.is_empty() {
            Some(self.current_player.other())
        } else {
            None
        }
    }

    pub fn moves_left(&self) -> u32 {
        self.moves_left
    }

    pub fn make_move(&mut self, pos: Pos) {
        let valid = self.is_valid_move(pos);
        if !valid {
            panic!(
                "Invalid move: {:?} ({:?} -> {:?})",
                pos,
                self.tile(pos),
                self.current_player
            );
        }

        board::make_move(&mut self.board, self.current_player, pos.x, pos.y);

        if !self.disable_heat_map {
            update_heat_map_incrementally(&mut self.heat_map, pos, self.current_player);

            if self.tile(pos).is_squashed() {
                // Squash move causes ownership change and possible branch disconnect,
                // full recompute is required for the other player tiles.
                update_heat_map_fully(self, self.current_player.other());
            }
        }

        let last = self.moves_left == 1;

        let left = if last {
            self.turn_length
        } else {
            self.moves_left - 1
        };

        self.moves_left = left;

        if last {
            self.current_player = self.current_player.other();
        }

        update_moves(self);
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
                let tile = self.board[x as usize][y as usize];
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
    use crate::board::base_pos;
    use crate::game::{GameState, TURN_LENGTH};
    use crate::models::Tile::Alive;
    use crate::models::{Player, Pos, Tile};
    use rand::seq::SliceRandom;

    impl Pos {
        pub fn new(x: u16, y: u16) -> Pos {
            Pos { x, y }
        }
    }

    #[test]
    fn create_game_returns_new_game_state() {
        let game = GameState::new();

        let tiles = game.board.iter().flat_map(|r| r.iter());
        for tile in tiles {
            match tile {
                Tile::Empty | Tile::Base(_) => {}
                Tile::Alive(_) => panic!("Alive tile on new board"),
                Tile::Squashed(_) => panic!("Squashed tile on new board"),
            }
        }

        assert_eq!(game.current_player, Player::Red);
        assert_eq!(game.turn_length as u8, TURN_LENGTH);
        assert_eq!(game.moves_left as u8, TURN_LENGTH);

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
        assert_eq!(
            game.board[pos.x as usize][pos.y as usize],
            Alive(game.current_player)
        );
    }

    #[test]
    fn make_random_move_fills_board_until_finished() {
        let mut game = GameState::new();
        game.disable_heat_map = true; // Reduce overhead: this test is heavy.
        let mut move_count = 0;

        loop {
            move_count += 1;
            match game.moves.choose(&mut rand::thread_rng()) {
                None => break,
                Some(&pos) => game.make_move(pos),
            }
        }

        println!("Total moves: {}", move_count);
        println!("{}", game);
    }

    #[test]
    fn create_game_returns_new_game_state_with_heat_map() {
        let game = GameState::new();
        let pos = base_pos(Player::Red);

        assert_eq!(game.heat(pos).red, (TURN_LENGTH + 1) as u8);

        assert_eq!(game.heat(pos).blue, 0);

        assert_eq!(
            game.heat(Pos {
                x: pos.x + 1,
                y: pos.y
            })
            .red,
            TURN_LENGTH as u8
        );

        assert_eq!(
            game.heat(Pos {
                x: pos.x + 2,
                y: pos.y
            })
            .red,
            (TURN_LENGTH - 1) as u8
        );

        assert_eq!(
            game.heat(Pos {
                x: pos.x + TURN_LENGTH as u16,
                y: pos.y
            })
            .red,
            1
        );

        assert_eq!(
            game.heat(Pos {
                x: pos.x + TURN_LENGTH as u16 + 1,
                y: pos.y
            })
            .red,
            0
        );
    }
}
