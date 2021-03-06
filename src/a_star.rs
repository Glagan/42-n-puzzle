use crate::puzzle::Puzzle;
use npuzzle::{neighbors, HeuristicFn, Mode, NodeWithCost, Solution};
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

fn reconstruct_path(paths: &HashMap<Vec<i32>, Vec<i32>>, node: &[i32]) -> Vec<Vec<i32>> {
    let mut full_path = vec![Vec::from(node)];

    let mut current = node;
    while paths.contains_key(current) {
        current = &paths[current];
        full_path.push(Vec::from(current));
    }

    full_path.reverse();
    full_path
}

pub fn solve(puzzle: &Puzzle, mode: &str, heuristic: HeuristicFn) -> Result<Solution, String> {
    let now = Instant::now();
    let mode = match mode {
        "greedy" => Mode::Greedy,
        "uniform" => Mode::Uniform,
        _ => Mode::Normal,
    };

    // Summary
    let mut total_used_states = 0;
    let mut biggest_state: usize = 1; // 1 is the initial state

    // State
    let mut open_set = BinaryHeap::new();
    open_set.push(NodeWithCost {
        cost: 0.,
        node: puzzle.map.clone(),
    });
    let mut open_set_ref: HashMap<Vec<i32>, bool> = HashMap::new();
    open_set_ref.insert(puzzle.map.clone(), true);
    // Keep a reference of already visited nodes to ignore
    // -- since heuristics needs to be admissibles revisiting a node is unnecessary
    let mut closed: HashMap<Vec<i32>, bool> = HashMap::new();
    // cameFrom -- best previous path to a node
    let mut best_path_to_node: HashMap<Vec<i32>, Vec<i32>> = HashMap::new();
    // gScore -- cost of the best path to a node
    let mut best_cost_to_node: HashMap<Vec<i32>, f64> = HashMap::new();
    best_cost_to_node.insert(puzzle.map.clone(), 0.);

    // Iterate on each cells
    while let Some(current) = open_set.pop() {
        open_set_ref.remove_entry(&current.node);
        closed.insert(current.node.clone(), true);
        total_used_states += 1;

        // Check if it's the goal
        if current.node == puzzle.goal {
            return Ok(Solution {
                total_used_states,
                biggest_state,
                steps: reconstruct_path(&best_path_to_node, &current.node),
            });
        }

        for neighbor in neighbors(puzzle.size, &current.node).into_iter().flatten() {
            // Ignore nodes already in closed set
            if closed.contains_key(&neighbor) {
                continue;
            }

            let next_move_cost = best_cost_to_node.get(&current.node).unwrap() + 1.;
            let neighbor_previous_cost = best_cost_to_node.get(&neighbor);
            // Check the node only if it was never checked or if it has a better cost than the last found
            if neighbor_previous_cost.is_none()
                || next_move_cost < *(neighbor_previous_cost.unwrap())
            {
                // Update each state hash maps
                if !best_path_to_node.contains_key(&neighbor) {
                    best_path_to_node.insert(neighbor.clone(), current.node.clone());
                } else if best_path_to_node[&neighbor] != current.node {
                    *best_path_to_node.get_mut(&neighbor).unwrap() = current.node.clone();
                }
                if !best_cost_to_node.contains_key(&neighbor) {
                    best_cost_to_node.insert(neighbor.clone(), next_move_cost);
                } else {
                    *best_cost_to_node.get_mut(&neighbor).unwrap() = next_move_cost;
                }
                // Add to open_set if it's not already inside
                if !open_set_ref.contains_key(&neighbor) {
                    open_set_ref.insert(neighbor.clone(), true);
                    open_set.push(NodeWithCost {
                        cost: match mode {
                            Mode::Normal => {
                                next_move_cost + heuristic(puzzle.size, &neighbor, &puzzle.goal)
                            }
                            Mode::Greedy => heuristic(puzzle.size, &neighbor, &puzzle.goal), // Ignore depth
                            Mode::Uniform => next_move_cost, // Ignore heuristic
                        },
                        node: neighbor,
                    });
                }
            }
        }

        if open_set.len() > biggest_state {
            biggest_state = open_set.len() + 1;
        }

        if total_used_states % 100000 == 0 {
            println!(
                "#> Explored {} states in {:.2?}",
                total_used_states,
                now.elapsed()
            );
        }
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
