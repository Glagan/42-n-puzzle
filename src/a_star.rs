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
    set.iter().any(|in_opened| *in_opened == *value)
}

pub fn solve(
    puzzle: &Puzzle,
    goal: &Node,
    heuristic: fn(&Node, &Node) -> i32,
) -> Result<Vec<Node>, String> {
    let now = Instant::now();

    // State
    let mut open_set: Vec<Node> = vec![puzzle.map.clone()];
    // cameFrom
    let mut best_path_to_node: HashMap<Node, Node> = HashMap::new();
    // gScore
    let mut best_score_to_node: HashMap<Node, i32> = HashMap::new();
    best_score_to_node.insert(puzzle.map.clone(), 0);
    // fScore
    let mut best_guess_to_node: HashMap<Node, i32> = HashMap::new();
    best_guess_to_node.insert(puzzle.map.clone(), heuristic(&puzzle.map, goal));

    // Iterate on each cells
    let mut it = 0;
    while !open_set.is_empty() {
        // open_set is sorted by best_guess_to_node and the first element is always the lowest
        let current = open_set.remove(0);

        // Check if it's the goal
        if current == *goal {
            return Ok(reconstruct_path(&best_path_to_node, &current));
        }

        for neighbor in neighbors(puzzle.size, &current).into_iter().flatten() {
            // println!("# checking {:#?}", neighbor);
            let goal_distance_for_neighbor = best_score_to_node.get(&current).unwrap() + 1;
            let neighbor_previous_score = best_score_to_node.get(&neighbor);
            // println!("# checking {:#?}", neighbor);
            if neighbor_previous_score.is_none()
                || goal_distance_for_neighbor < *neighbor_previous_score.unwrap()
            {
                best_path_to_node
                    .entry(neighbor.clone())
                    .or_insert_with(|| current.clone());
                best_score_to_node
                    .entry(neighbor.clone())
                    .or_insert(goal_distance_for_neighbor);
                let neighbor_distance_to_goal =
                    goal_distance_for_neighbor + heuristic(&neighbor, goal);
                // println!("h(x) = {}", neighbor_distance_to_goal);
                best_guess_to_node
                    .entry(neighbor.clone())
                    .or_insert(neighbor_distance_to_goal);
                if !set_has_node(&open_set, &neighbor) {
                    match set_best_position(
                        &open_set,
                        &best_guess_to_node,
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
            // println!("{:#?}", best_score_to_node);
            println!("it #{} elapsed {:.2?}", it, now.elapsed());
        }
        it += 1;
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
