use crate::board;
use crate::board::dist;
use crate::game::GameState;
use crate::models::{Player, Pos, Tile, TilePos, BOARD_HEIGHT, BOARD_WIDTH};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct PosCost {
    pos: Pos,
    cost: u32,
}

impl Ord for PosCost {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip order on cost.
        // Compare on x & y too for consistency across Ord and PartialEq.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
    }
}

impl PartialOrd for PosCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn moves(game: &GameState) -> impl Iterator<Item = TilePos> + '_ {
    game.moves().iter().map(move |&pos| TilePos {
        pos,
        tile: game.tile(pos),
    })
}

pub fn get_ai_move(game: &GameState) -> Option<Pos> {
    if game.moves().is_empty() {
        return Option::None;
    }

    attack_move(game).or_else(|| advance_move(game))
}

fn attack_move(game: &GameState) -> Option<Pos> {
    // TODO: Find a tile to cut them off better - maximize enemy cost to reach our base.
    moves(game).find(|&t| t.tile.is_alive()).map(|t| t.pos)
}

fn advance_move(game: &GameState) -> Option<Pos> {
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
    if let Some(pos) = find_path(
        game,
        game.current_player(),
        game.current_base(),
        game.enemy_base(),
    )
    .and_then(|i| i.filter(|&p| game.is_valid_move(p)).last())
    {
        return Some(pos);
    }

    // Path not found, just do something, we have probably lost at this point.
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

    let nonempty_neighbs = board::neighbors(pos)
        .filter(|n| !game.tile(*n).is_empty())
        .count();

    let diag_neighbs = board::neighbors(pos)
        .filter(|n| n.x != pos.x && n.y != pos.y && !game.tile(*n).is_empty())
        .count();

    let nondiag_neighbs = nonempty_neighbs - diag_neighbs;

    let is_edge =
        if pos.x == 0 || pos.y == 0 || pos.x == BOARD_WIDTH - 1 || pos.y == BOARD_HEIGHT - 1 {
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

/// Finds cheapest path between two positions with A* algorithm, using board::dist() as heuristic.
fn find_path(
    game: &GameState,
    player: Player,
    start: Pos,
    end: Pos,
) -> Option<impl Iterator<Item = Pos>> {
    if start == end {
        return None;
    }

    // List of nodes to visit.
    let mut heap = BinaryHeap::new();
    heap.push(PosCost {
        pos: start,
        cost: dist(start, end) as u32,
    });

    // List of visited nodes.
    let mut visited = [[false; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

    // "Parent" nodes map - allows us to reconstruct the path from start to given pos.
    let mut came_from = [[None as Option<Pos>; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score = [[std::u64::MAX; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];
    g_score[start.x as usize][start.y as usize] = 0;

    while let Some(current) = heap.pop() {
        visited[current.pos.x as usize][current.pos.y as usize] = true;

        for neighb in board::neighbors(current.pos) {
            if neighb == end {
                // Target reached, return results.
                let mut res_pos = current.pos;

                let iter = std::iter::from_fn(move || {
                    if let Some(prev) = came_from[res_pos.x as usize][res_pos.y as usize] {
                        let res = Some(res_pos);
                        res_pos = prev;
                        return res;
                    }

                    None
                });

                /*
                // TEMP: Print costs
                let mut res = String::new();

                #[allow(clippy::needless_range_loop)]
                for y in 0..BOARD_HEIGHT {
                    for x in 0..BOARD_WIDTH {
                        let cost = g_score[x as usize][y as usize];
                        if cost == std::u64::MAX {
                            res.push_str(" ?  ");
                        } else {
                            res.push_str(format!("{:^3} ", cost).as_str());
                        };
                    }
                    res.push('\n');
                }

                println!("{}", res);
                // END
                */

                return Some(iter);
            }

            let tile = game.tile(neighb);

            let neighb_cost = match tile {
                // Moving into empty tile is not as good as squashing.
                Tile::Empty => (1 + weight(game, neighb, false)) * 3,

                // Tile belongs to the player and does not cost anything.
                Tile::Alive(p) | Tile::Squashed(p) if p == player => 0,

                // Other player tile is at min cost, squashing is preferred.
                Tile::Alive(p) if p == player.other() => 1,

                // All the rest is forbidden.
                _ => std::u16::MAX,
            };

            if neighb_cost == std::u16::MAX {
                continue;
            }

            let cur_score = g_score[current.pos.x as usize][current.pos.y as usize];
            let neighb_score = cur_score + neighb_cost as u64;
            let old_neighb_score = g_score[neighb.x as usize][neighb.y as usize];

            if neighb_score < old_neighb_score {
                // Found a better path through neigb, record it.
                came_from[neighb.x as usize][neighb.y as usize] = Some(current.pos);
                g_score[neighb.x as usize][neighb.y as usize] = neighb_score;

                if !visited[neighb.x as usize][neighb.y as usize] {
                    heap.push(PosCost {
                        pos: neighb,
                        cost: neighb_score as u32 + dist(neighb, end) as u32,
                    });
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::ai::{find_path, get_ai_move};
    use crate::models::{Player, Pos};
    use crate::{board, game};
    use rand::seq::IteratorRandom;

    #[test]
    fn ai_wins_against_random() {
        let mut game = game::GameState::new();
        let ai_player = Player::Red;

        while !game.moves().is_empty() {
            let pos = if game.current_player() == ai_player {
                get_ai_move(&game)
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

        let path: Vec<Pos> = find_path(&game, player, start, end)
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
        let start = game.current_base();
        let end = game.enemy_base();
        let player = game.current_player();

        let mut path: Vec<Pos> = find_path(&game, player, start, end)
            .expect("path is expected")
            .collect();

        path.reverse();

        for pos in path {
            game.make_move(pos);
        }

        println!("{}", game)
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

        let path: Vec<Pos> = find_path(&game, player, start, end)
            .expect("path is expected")
            .collect();

        assert!(path.is_empty());
    }

    #[test]
    fn find_path_returns_none_for_same_start_end() {
        let game = game::GameState::new();
        let pos = game.current_base();
        let path = find_path(&game, game.current_player(), pos, pos);

        assert!(path.is_none());
    }
}
