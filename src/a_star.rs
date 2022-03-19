use crate::puzzle::Puzzle;
use npuzzle::{neighbors, Node};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

#[derive(Clone)]
struct NodeWithCost {
    cost: f64,
    node: Node,
}

impl Eq for NodeWithCost {}

impl PartialEq for NodeWithCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap().reverse()
    }
}

impl PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}

pub struct Solution {
    pub total_used_states: i32,
    pub biggest_state: usize,
    pub steps: Vec<Node>,
}

fn reconstruct_path(paths: &HashMap<Node, Node>, node: &Node) -> Vec<Node> {
    let mut full_path = vec![node.clone()];

    let mut current = node;
    while paths.contains_key(current) {
        current = &paths[current];
        full_path.push(current.clone());
    }

    full_path.reverse();
    full_path
}

pub fn solve(
    puzzle: &Puzzle,
    mode: &str,
    heuristic: fn(i32, &Node, &Node) -> f64,
) -> Result<Solution, String> {
    let now = Instant::now();
    let mode = match mode {
        "greedy" => 1,
        "uniform" => 2,
        _ => 0,
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
    // cameFrom -- best previous path to a node
    let mut best_path_to_node: HashMap<Node, Node> = HashMap::new();
    // gScore -- cost of the best path to a node
    let mut best_cost_to_node: HashMap<Node, f64> = HashMap::new();
    best_cost_to_node.insert(puzzle.map.clone(), 0.);

    // Iterate on each cells
    while let Some(current) = open_set.pop() {
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
                if !open_set.iter().any(|node| node.node == neighbor) {
                    open_set.push(NodeWithCost {
                        cost: match mode {
                            0 => next_move_cost + heuristic(puzzle.size, &neighbor, &puzzle.goal),
                            1 => heuristic(puzzle.size, &neighbor, &puzzle.goal), // Ignore depth
                            _ => next_move_cost, // Ignore heuristic
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
                "# Selected {} states in {:.2?}",
                total_used_states,
                now.elapsed()
            );
        }
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
