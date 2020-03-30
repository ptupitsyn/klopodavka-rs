use crate::game::GameState;
use crate::models::Tile::Squashed;
use crate::models::{Player, Pos, Tile, TilePos};
use crate::{board, path};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AiMode {
    Basic,
    Advanced,
}

pub fn moves(game: &GameState) -> impl Iterator<Item = TilePos> + '_ {
    game.moves().iter().map(move |&pos| TilePos {
        pos,
        tile: game.tile(pos),
    })
}

pub fn get_ai_move(game: &GameState) -> Option<Pos> {
    get_ai_move_with_mode(game, AiMode::Advanced)
}

pub fn get_ai_move_with_mode(game: &GameState, mode: AiMode) -> Option<Pos> {
    // TODO: Returns all moves for the current turn at once.
    if game.moves().is_empty() {
        return Option::None;
    }

    attack_move(game, mode).or_else(|| advance_move(game, mode))
}

fn attack_move(game: &GameState, mode: AiMode) -> Option<Pos> {
    if mode == AiMode::Basic {
        return game
            .moves()
            .iter()
            .find(|&&p| game.tile(p).is_alive())
            .copied();
    }

    let player = game.current_player();
    let enemy = player.other();

    // TODO: Use better cost instead of count()
    let enemy_cost = |p| {
        path::find_path_ex(
            game,
            enemy,
            game.enemy_base(),
            game.current_base(),
            Some(TilePos {
                pos: p,
                tile: Squashed(player),
            }),
            cost,
        )
        .map_or(std::u32::MAX, |p| p.count() as u32)
    };

    // Use heat map to find reachable tile that is the most important for the enemy,
    // then squash that tile.
    game.tiles()
        .filter(|&t| t.tile == Tile::Alive(enemy) && game.heat(t.pos).get(player) > 0)
        .max_by(|a, b| enemy_cost(a.pos).cmp(&enemy_cost(b.pos)))
        .and_then(|p| path::find_path_ex(game, player, game.current_base(), p.pos, None, cost))
        .and_then(|iter| iter.last())
}

fn advance_move(game: &GameState, mode: AiMode) -> Option<Pos> {
    if mode == AiMode::Advanced {
        let has_squashed = game.tiles().any(|t| t.tile.is_squashed());

        if !has_squashed {
            // Return random diagonal move when fight has not yet started.
            // Note: this is quite slow.
            let moves: Vec<&Pos> = game
                .moves()
                .iter()
                .filter(|&&pos| weight(game, pos, false) == 1)
                .collect();

            if !moves.is_empty() {
                // There is no Random or Time on wasm-unknown target (???), they panic,
                // use this sum as a pseudo-random number.
                let empty_tiles_pos_sum: usize = game
                    .tiles()
                    .filter(|t| t.tile.is_empty())
                    .map(|t| t.pos.y as usize + t.pos.x as usize)
                    .sum();

                return moves
                    .get(empty_tiles_pos_sum % moves.len())
                    .copied()
                    .copied();
            }
        }

        // Rush to enemy base with path finding.
        if let Some(pos) = path::find_path_ex(
            game,
            game.current_player(),
            game.current_base(),
            game.enemy_base(),
            None,
            cost,
        )
        .and_then(|i| i.filter(|&p| game.is_valid_move(p)).last())
        {
            return Some(pos);
        }
    }

    game.moves()
        .iter()
        .min_by(|&&x, &&y| weight(game, x, true).cmp(&weight(game, y, true)))
        .copied()
}

fn weight(game: &GameState, pos: Pos, include_base_dist: bool) -> u16 {
    // Compute move weights based on:
    // * Neighbor count - less is better
    // * Diagonal - true is better
    // * Enemy base distance - less is better
    let size = game.size();

    let nonempty_neighbs = board::neighbors(pos, size)
        .filter(|n| !game.tile(*n).is_empty())
        .count();

    let diag_neighbs = board::neighbors(pos, size)
        .filter(|n| n.x != pos.x && n.y != pos.y && !game.tile(*n).is_empty())
        .count();

    let nondiag_neighbs = nonempty_neighbs - diag_neighbs;

    let is_edge = if pos.x == 0 || pos.y == 0 || pos.x == size.width - 1 || pos.y == size.height - 1
    {
        1
    } else {
        0
    };

    let base_dist = if include_base_dist {
        let enemy_base = game.enemy_base();
        board::dist(enemy_base, pos)
    } else {
        0
    };

    let weight = diag_neighbs + nondiag_neighbs * 2 + base_dist as usize + is_edge;

    weight as u16
}

fn cost(game: &GameState, pos: Pos, tile: Tile, player: Player) -> u16 {
    match tile {
        Tile::Empty => (1 + weight(game, pos, false)) * 3,
        _ => path::cost_default(game, pos, tile, player),
    }
}

#[cfg(test)]
mod tests {
    use crate::ai::{cost, get_ai_move_with_mode, AiMode};
    use crate::models::{Player, Pos};
    use crate::path::{find_path, find_path_ex};
    use crate::{board, game};
    use rand::seq::IteratorRandom;

    #[test]
    fn basic_ai_wins_against_random() {
        let mut game = game::GameState::new();
        let ai_player = Player::Red;

        while !game.moves().is_empty() {
            let pos = if game.current_player() == ai_player {
                get_ai_move_with_mode(&game, AiMode::Basic)
                    .expect("get_ai_move returns something when game.moves() is not empty")
            } else {
                *game
                    .moves()
                    .iter()
                    .choose(&mut rand::thread_rng())
                    .expect("random")
            };

            game.make_move(pos);
        }

        // AI opponent has no more moves left => AI won.
        assert_eq!(game.winner().expect("some winner"), ai_player);
    }

    #[test]
    fn find_path_works_on_empty_board() {
        let game = game::GameState::new();
        let start = game.current_base();
        let end = game.enemy_base();
        let player = game.current_player();

        let path: Vec<Pos> = find_path_ex(&game, player, start, end, None, cost)
            .expect("path is expected")
            .collect();

        assert!(!path.is_empty());
        assert_eq!(path.len() + 1, board::dist(start, end) as usize);
        assert!(!path.contains(&start));
        assert!(!path.contains(&end));

        let path_start = path.last().copied().expect("path start is expected");
        let path_end = path.first().copied().expect("path end is expected");

        assert_eq!(board::dist(start, path_start), 1);
        assert_eq!(board::dist(end, path_end), 1);
    }

    #[test]
    fn find_path_returns_valid_moves() {
        let mut game = game::GameState::new_custom(1000);

        let mut path: Vec<Pos> = find_path(&game).expect("path is expected").collect();

        path.reverse();

        for pos in path {
            game.make_move(pos);
        }
    }

    #[test]
    fn find_path_returns_empty_iter_for_adjacent_tiles() {
        let game = game::GameState::new();
        let start = game.current_base();
        let end = Pos {
            x: start.x,
            y: start.y + 1,
        };
        let player = game.current_player();

        let path: Vec<Pos> = find_path_ex(&game, player, start, end, None, cost)
            .expect("path is expected")
            .collect();

        assert!(path.is_empty());
    }

    #[test]
    fn find_path_returns_none_for_same_start_end() {
        let game = game::GameState::new();
        let pos = game.current_base();
        let path = find_path_ex(&game, game.current_player(), pos, pos, None, cost);

        assert!(path.is_none());
    }
}
