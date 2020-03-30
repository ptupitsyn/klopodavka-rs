use crate::board;
use crate::game::GameState;
use crate::models::*;
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

// TODO: Move path finding to a separate module, so it can be used for things like
// path highlighting and win detection within the game.
pub fn find_path(game: &GameState) -> Option<impl Iterator<Item = Pos>> {
    find_path_ex(
        game,
        game.current_player(),
        game.current_base(),
        game.enemy_base(),
        None,
        cost_default,
    )
}

/// Finds cheapest path between two positions with A* algorithm, using board::dist() as heuristic.
pub fn find_path_ex(
    game: &GameState,
    player: Player,
    start: Pos,
    end: Pos,
    tile_override: Option<TilePos>,
    cost_fn: impl Fn(&GameState, Pos, Tile, Player) -> u16,
) -> Option<impl Iterator<Item = Pos>> {
    if start == end {
        return None;
    }

    // List of nodes to visit.
    let mut heap = BinaryHeap::new();
    heap.push(PosCost {
        pos: start,
        cost: board::dist(start, end) as u32,
    });

    // List of visited nodes.
    let size = game.size();
    let mut visited: Tiles<bool> = Tiles::with_size(size);

    // "Parent" nodes map - allows us to reconstruct the path from start to given pos.
    let mut came_from: Tiles<Option<Pos>> = Tiles::with_size(size);

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: Tiles<u64> = Tiles::with_size_and_val(size, std::u64::MAX);
    g_score[start] = 0;

    while let Some(current) = heap.pop() {
        visited[current.pos] = true;

        for neighb in board::neighbors(current.pos, size) {
            if neighb == end {
                // Target reached, return results.
                let mut res_pos = current.pos;

                let iter = std::iter::from_fn(move || {
                    if let Some(prev) = came_from[res_pos] {
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

            let tile = match tile_override {
                Some(t) if t.pos == neighb => t.tile,
                _ => game.tile(neighb),
            };

            let neighb_cost = cost_fn(game, neighb, tile, player);

            if neighb_cost == std::u16::MAX {
                continue;
            }

            let cur_score = g_score[current.pos];
            let neighb_score = cur_score + neighb_cost as u64;
            let old_neighb_score = g_score[neighb];

            if neighb_score < old_neighb_score {
                // Found a better path through neigb, record it.
                came_from[neighb] = Some(current.pos);
                g_score[neighb] = neighb_score;

                if !visited[neighb] {
                    heap.push(PosCost {
                        pos: neighb,
                        cost: neighb_score as u32 + board::dist(neighb, end) as u32,
                    });
                }
            }
        }
    }

    None
}

pub fn cost_default(_game: &GameState, _pos: Pos, tile: Tile, player: Player) -> u16 {
    match tile {
        // Moving into empty tile is not as good as squashing.
        Tile::Empty => 3,

        // Tile belongs to the player and does not cost anything.
        Tile::Alive(p) | Tile::Squashed(p) if p == player => 0,

        // Other player tile is at min cost, squashing is preferred.
        Tile::Alive(p) if p == player.other() => 1,

        // All the rest is forbidden.
        _ => std::u16::MAX,
    }
}
