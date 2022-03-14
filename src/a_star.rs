use crate::puzzle::Puzzle;
use npuzzle::{neighbors, Node};
use std::collections::HashMap;
use std::time::Instant;

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

// fn best_current_node(set: &[Node], scores: &HashMap<Node, i32>) -> usize {
//     let mut best_index = 0;
//     let mut best_score = i32::MAX;

//     for (index, node) in set.iter().enumerate() {
//         let score = if scores.contains_key(node) {
//             scores[node]
//         } else {
//             i32::MAX
//         };
//         if score < best_score {
//             best_index = index;
//             best_score = score;
//         }
//     }

//     best_index
// }

fn set_best_position(set: &[Node], scores: &HashMap<Node, i32>, score: i32) -> Option<usize> {
    for (index, node) in set.iter().enumerate() {
        let node_score = if scores.contains_key(node) {
            scores[node]
        } else {
            i32::MAX - 1
        };
        if score < node_score {
            return Some(index);
        }
    }

    None
}

fn set_has_node(set: &[Node], value: &Node) -> bool {
    set.contains(value)
}

pub fn solve(
    puzzle: &Puzzle,
    goal: &Node,
    heuristic: fn(&Node, &Node) -> i32,
) -> Result<Vec<Node>, String> {
    let now = Instant::now();

    // State
    let mut open_set: Vec<Node> = vec![puzzle.map.clone()];
    // cameFrom -- best previous path to a node
    let mut best_path_to_node: HashMap<Node, Node> = HashMap::new();
    // gScore -- cost of the best path to a node
    let mut best_cost_to_node: HashMap<Node, i32> = HashMap::new();
    best_cost_to_node.insert(puzzle.map.clone(), 0);
    // fScore -- cost of the current best found path to a node
    let mut total_cost_to_node: HashMap<Node, i32> = HashMap::new();
    total_cost_to_node.insert(puzzle.map.clone(), heuristic(&puzzle.map, goal));

    // Iterate on each cells
    let mut it = 0;
    while !open_set.is_empty() {
        // open_set is not sorted
        // let best_index = best_current_node(&open_set, &total_cost_to_node);
        // let current = open_set.remove(best_index);
        // open_set is sorted by best_guess_to_node and the first element is always the lowest
        let current = open_set.remove(0);

        // Check if it's the goal
        if current == *goal {
            return Ok(reconstruct_path(&best_path_to_node, &current));
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
                let neighbor_distance_to_goal = next_move_cost + heuristic(&neighbor, goal);
                if !total_cost_to_node.contains_key(&neighbor) {
                    total_cost_to_node.insert(neighbor.clone(), neighbor_distance_to_goal);
                } else {
                    *total_cost_to_node.get_mut(&neighbor).unwrap() = neighbor_distance_to_goal;
                }
                // println!("h(x) = {}", neighbor_distance_to_goal);
                if !set_has_node(&open_set, &neighbor) {
                    // open_set.push(neighbor.clone());
                    match set_best_position(
                        &open_set,
                        &total_cost_to_node,
                        neighbor_distance_to_goal,
                    ) {
                        Some(index) => open_set.insert(index, neighbor.clone()),
                        None => open_set.push(neighbor.clone()),
                    }
                }
            }
        }

        // println!("it {}", it);
        if it % 1000 == 0 {
            // println!("{:#?}", best_cost_to_node);
            println!("it #{} elapsed {:.2?}", it, now.elapsed());
        }
        it += 1;
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
