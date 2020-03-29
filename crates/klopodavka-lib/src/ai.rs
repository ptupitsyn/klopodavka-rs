use crate::board;
use crate::board::dist;
use crate::game::GameState;
use crate::models::Tile::Squashed;
use crate::models::{Player, Pos, Tile, TilePos, Tiles, BOARD_HEIGHT, BOARD_WIDTH};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::intrinsics::transmute;

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

pub fn get_ai_move(game: &GameState) -> Option<TilePos> {
    if game.moves().is_empty() {
        return Option::None;
    }

    attack_move(game).or_else(|| advance_move(game))
}

fn attack_move(game: &GameState) -> Option<TilePos> {
    // TODO: Find a tile to cut them off better - maximize enemy cost to reach our base.
    moves(game).find(|&t| t.tile.is_alive())
}

fn advance_move(game: &GameState) -> Option<TilePos> {
    let has_squashed = game.tiles().any(|t| t.tile.is_squashed());

    if !has_squashed {
        // Return random diagonal move when fight has not yet started.
        // Note: this is quite slow.
        let moves: Vec<TilePos> = moves(game)
            .filter(|m| weight(game, m.pos, false) == 1)
            .collect();

        if !moves.is_empty() {
            // There is no Random or Time on wasm-unknown target (???), they panic,
            // use this sum as a pseudo-random number.
            let empty_tiles_pos_sum: usize = game
                .tiles()
                .filter(|t| t.tile.is_empty())
                .map(|t| t.pos.y as usize + t.pos.x as usize)
                .sum();

            return moves.get(empty_tiles_pos_sum % moves.len()).copied();
        }
    }

    // TODO: Squashed are present. Rush to enemy base with A*
    // * Maximize squashes
    // * Minimize new clops
    // * Minimize contact with squashed enemy tiles.
    moves(game).min_by(|&x, &y| weight(game, x.pos, true).cmp(&weight(game, y.pos, true)))
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

    // TODO: Base positions should be saved within GameState
    let base_dist = if include_base_dist {
        let enemy_base = board::base_pos(game.current_player().other());
        board::dist(enemy_base, pos)
    } else {
        0
    };

    let weight = diag_neighbs + nondiag_neighbs * 2 + base_dist as usize + is_edge;

    weight as u16
}

/// Finds cheapest path between two positions.
fn find_path(game: &GameState, player: Player, start: Pos, end: Pos) -> Option<Vec<Pos>> {
    // List of nodes to visit.
    let mut heap = BinaryHeap::new();
    heap.push(PosCost {
        pos: start,
        cost: dist(start, end) as u32,
    });

    // List of visited nodes.
    let mut visited = [[false; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];

    // "Parent" nodes map - allows us to reconstruct the path from start to given pos.
    // TODO: Array
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score = [[std::u64::MAX; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize];
    g_score[start.x as usize][start.y as usize] = 0;

    while let Some(current) = heap.pop() {
        if current.pos == end {
            // Return results
            let mut res: Vec<Pos> = Vec::new();
            res.push(current.pos);

            let mut p = current.pos;

            while let Some(&from) = came_from.get(&p) {
                res.push(from);
                p = from;
            }

            return Some(res);
        }

        visited[current.pos.x as usize][current.pos.y as usize] = true;

        for neighb in board::neighbors(current.pos) {
            let tile = game.tile(neighb);

            let neighb_cost = match tile {
                // Moving into empty tile is not as good as squashing.
                Tile::Empty => weight(game, neighb, false) * 3,

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

            // TODO: Exclude invalid moves (squashed tiles, bases, etc).
            let cur_score = g_score[current.pos.x as usize][current.pos.y as usize];
            // TODO: According to tile type (squash is cheap, new clop is expensive, depending on neighbor count, etc)
            let neighb_score = cur_score + neighb_cost as u64;
            let old_neighb_score = g_score[neighb.x as usize][neighb.y as usize];

            if neighb_score < old_neighb_score {
                // Found a better path through neigb, record it.
                came_from.insert(neighb, current.pos);
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
    use crate::ai::get_ai_move;
    use crate::game;
    use crate::models::Player;
    use rand::seq::IteratorRandom;

    #[test]
    fn ai_wins_against_random() {
        let mut game = game::GameState::new();
        let ai_player = Player::Red;

        while !game.moves().is_empty() {
            let pos = if game.current_player() == ai_player {
                get_ai_move(&game)
                    .expect("get_ai_move returns something when game.moves() is not empty")
                    .pos
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
}
