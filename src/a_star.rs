use crate::puzzle::Puzzle;
use npuzzle::{neighbors, Node};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

#[derive(Clone, Eq, PartialEq)]
struct NodeWithCost {
    cost: i32,
    node: Node,
}

impl Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution {
    pub total_used_states: i32,
    pub biggest_state: i32,
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
    goal: &Node,
    heuristic: fn(&Node, &Node) -> i32,
) -> Result<Solution, String> {
    let now = Instant::now();

    // Summary
    let mut total_used_states = 0;
    let mut biggest_state: i32 = 0;

    // State
    let mut open_set = BinaryHeap::new();
    open_set.push(NodeWithCost {
        cost: 0,
        node: puzzle.map.clone(),
    });
    // cameFrom -- best previous path to a node
    let mut best_path_to_node: HashMap<Node, Node> = HashMap::new();
    // gScore -- cost of the best path to a node
    let mut best_cost_to_node: HashMap<Node, i32> = HashMap::new();
    best_cost_to_node.insert(puzzle.map.clone(), 0);

    // Iterate on each cells
    while let Some(NodeWithCost {
        node: current,
        cost: _,
    }) = open_set.pop()
    {
        total_used_states += 1;
        if open_set.len() > biggest_state.try_into().unwrap() {
            biggest_state = open_set.len().try_into().unwrap();
        }

        // Check if it's the goal
        if current == *goal {
            return Ok(Solution {
                total_used_states,
                biggest_state,
                steps: reconstruct_path(&best_path_to_node, &current),
            });
        }

        for neighbor in neighbors(puzzle.size, &current).into_iter().flatten() {
            // println!("# checking {:#?}", neighbor);
            let next_move_cost = best_cost_to_node.get(&current).unwrap() + 1;
            let neighbor_previous_cost = best_cost_to_node.get(&neighbor);
            // println!("# checking {:#?}", neighbor);
            if neighbor_previous_cost.is_none()
                || next_move_cost < *(neighbor_previous_cost.unwrap())
            {
                // Update each state hash maps
                if !best_path_to_node.contains_key(&neighbor) {
                    best_path_to_node.insert(neighbor.clone(), current.clone());
                } else if best_path_to_node[&neighbor] != current {
                    *best_path_to_node.get_mut(&neighbor).unwrap() = current.clone();
                }
                if !best_cost_to_node.contains_key(&neighbor) {
                    best_cost_to_node.insert(neighbor.clone(), next_move_cost);
                } else {
                    *best_cost_to_node.get_mut(&neighbor).unwrap() = next_move_cost;
                }
                // println!("h(x) = {}", neighbor_distance_to_goal);
                // if !open_set.iter().any(|node| node.node == neighbor) {
                open_set.push(NodeWithCost {
                    cost: next_move_cost + heuristic(&neighbor, goal),
                    node: neighbor.clone(),
                });
                // }
            }
        }

        // println!("it {}", it);
        if total_used_states % 1000 == 0 {
            // println!("{:#?}", best_cost_to_node);
            println!(
                "# Total number of states ever selected: {} in {:.2?}",
                total_used_states,
                now.elapsed()
            );
        }
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
