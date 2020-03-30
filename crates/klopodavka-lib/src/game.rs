use crate::board;
use crate::board::Board;
use crate::models::*;
use std::fmt;

pub const TURN_LENGTH: u8 = 6;

pub type BoolTiles = Tiles<bool>;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct HeatMapTile {
    pub red: u8,
    pub blue: u8,
}

impl HeatMapTile {
    pub fn get(self, player: Player) -> u8 {
        match player {
            Player::Red => self.red,
            Player::Blue => self.blue,
        }
    }

    fn set(&mut self, player: Player, heat: u8) {
        match player {
            Player::Red => self.red = heat,
            Player::Blue => self.blue = heat,
        }
    }
}

type HeatMap = Tiles<HeatMapTile>;

pub struct GameState {
    board: Board,
    current_player: Player,
    turn_length: u32,
    moves_left: u32,
    moves: Vec<Pos>,
    moves_map: BoolTiles,
    heat_map: HeatMap,
    pub disable_heat_map: bool,
}

fn update_moves(game: &mut GameState) {
    // Reuse existing vector.
    game.moves
        .splice(0.., board::moves(&game.board, game.current_player));

    // Reallocate map - faster than clean/update.
    game.moves_map = Tiles::with_size(game.board.size());

    for pos in game.moves.iter() {
        game.moves_map.setp(*pos, true);
    }
}

fn update_heat_map_incrementally(
    map: &mut HeatMap,
    board: &Board,
    max_val: u8,
    pos: Pos,
    player: Player,
) {
    map[pos].set(player, max_val);

    let mut visited: BoolTiles = Tiles::with_size(map.size());

    let mut pending: Vec<Pos> = Vec::new();
    pending.push(pos);

    let enemy = player.other();

    while let Some(pos) = pending.pop() {
        visited[pos] = true;
        let pos_heat = map[pos].get(player);

        let neighb_heat = match board[pos] {
            Tile::Empty => pos_heat - 1,
            Tile::Alive(pl) if pl == enemy => pos_heat - 1,
            Tile::Alive(pl) | Tile::Squashed(pl) | Tile::Base(pl) if pl == player => pos_heat,
            other => panic!("unexpected tile: {:?}", other),
        };

        // TODO: This is somehow wrong, because we sometimes find a better path to a given tile,
        // but that tile is already visited, so we don't update.
        // Because we must use a priority queue instead of a stack.
        // TODO: This is too similar to path finding, how can we reuse?
        // Provide different cost func, and limit path length, and pass cost map?
        for neighb in board::neighbors(pos, map.size()) {
            let tile = board[neighb];

            if neighb_heat > map[neighb].get(player)
                && tile != Tile::Squashed(enemy)
                && tile != Tile::Base(enemy)
            {
                map[neighb].set(player, neighb_heat);

                if !visited[neighb] {
                    pending.push(neighb);
                }
            }
        }
    }
}

fn update_heat_map_fully(game: &mut GameState, player: Player) {
    let map = &mut game.heat_map;

    for pos in board::pos_iter(map.size()) {
        match player {
            Player::Red => map[pos].red = 0,
            Player::Blue => map[pos].blue = 0,
        };
    }

    for connected_tile_pos in board::connected_tiles(&game.board, player, false) {
        update_heat_map_incrementally(
            &mut game.heat_map,
            &game.board,
            game.turn_length as u8,
            connected_tile_pos,
            player,
        );
    }
}

fn new_moves_map(size: Size) -> BoolTiles {
    Tiles::with_size(size)
}

#[allow(clippy::new_without_default)]
impl GameState {
    pub fn new() -> Self {
        GameState::new_custom(TURN_LENGTH as u32)
    }

    pub fn new_custom(turn_length: u32) -> Self {
        let tiles = board::create_board();
        let size = tiles.size();
        let player = Player::Red;

        let mut res = GameState {
            board: tiles,
            current_player: player,
            turn_length,
            moves_left: turn_length,
            moves_map: new_moves_map(size),
            moves: Vec::with_capacity(64),
            heat_map: Tiles::with_size(size),
            disable_heat_map: false,
        };

        update_heat_map_fully(&mut res, Player::Red);
        update_heat_map_fully(&mut res, Player::Blue);

        update_moves(&mut res);

        res
    }

    pub fn tile(&self, pos: Pos) -> Tile {
        self.board[pos]
    }

    pub fn heat(&self, pos: Pos) -> HeatMapTile {
        self.heat_map[pos]
    }

    pub fn max_heat(&self) -> u8 {
        self.turn_length as u8
    }

    pub fn tiles(&self) -> impl Iterator<Item = TilePos> + '_ {
        board::pos_iter(self.board.size()).map(move |pos| TilePos {
            pos,
            tile: self.tile(pos),
        })
    }

    pub fn moves(&self) -> &Vec<Pos> {
        self.moves.as_ref()
    }

    pub fn is_valid_move(&self, pos: Pos) -> bool {
        self.moves_map[pos]
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn current_base(&self) -> Pos {
        // TODO: Cache?
        board::base_pos(self.current_player, self.board.size())
    }

    pub fn enemy_base(&self) -> Pos {
        board::base_pos(self.current_player.other(), self.board.size())
    }

    pub fn size(&self) -> Size {
        self.board.size()
    }

    pub fn winner(&self) -> Option<Player> {
        // TODO: Early detection with pathfinder.
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
        let player = self.current_player;

        let valid = self.is_valid_move(pos);
        if !valid {
            panic!(
                "Invalid move: {:?} ({:?} -> {:?})",
                pos,
                self.tile(pos),
                player
            );
        }

        board::make_move(&mut self.board, player, pos.x, pos.y);

        if !self.disable_heat_map {
            update_heat_map_incrementally(
                &mut self.heat_map,
                &self.board,
                self.turn_length as u8,
                pos,
                player,
            );

            if self.tile(pos).is_squashed() {
                // Squash move causes ownership change and possible branch disconnect,
                // full recompute is required for the other player tiles.
                update_heat_map_fully(self, player.other());
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
            self.current_player = player.other();
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
        for y in 0..self.board.size().height {
            for x in 0..self.board.size().width {
                let tile = self.board.get(x, y).expect("valid tile");
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

        let tiles = game.tiles();
        for tile in tiles {
            match tile.tile {
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

        game.make_move(game.current_base());
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
        let base_pos = game.current_base();
        let pos = Pos::new(base_pos.x, base_pos.y + 1);
        game.make_move(pos);

        assert_eq!(game.moves_left, game.turn_length - 1);
        assert_eq!(game.board[pos], Alive(game.current_player));
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
        let pos = base_pos(Player::Red, game.board.size());

        assert_eq!(game.heat(pos).red, game.turn_length as u8);

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
