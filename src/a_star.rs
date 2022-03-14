use crate::puzzle::Puzzle;
use npuzzle::{neighbors, Node};
use std::collections::HashMap;

struct NodeWithScore {
    pub score: i32,
    pub node: Node,
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

// TODO: Priority queue
fn best_current_node(set: &[NodeWithScore]) -> usize {
    let mut best_index = 0;
    let mut best_score = i32::MAX;

    for (index, node) in set.iter().enumerate() {
        if node.score < best_score {
            best_index = index;
            best_score = node.score;
        }
    }

    best_index
}

fn set_best_position(set: &[NodeWithScore], value: i32) -> usize {
    let mut best_index = 0;

    for (index, node) in set.iter().enumerate() {
        if value < node.score {
            best_index = index;
        }
    }

    best_index
}

fn set_has_node(set: &[NodeWithScore], value: &Node) -> bool {
    set.iter().any(|in_opened| in_opened.node == *value)
}

pub fn solve(
    puzzle: &Puzzle,
    goal: &Node,
    heuristic: fn(&Node, &Node) -> i32,
) -> Result<Vec<Node>, String> {
    // State
    let mut open_set: Vec<NodeWithScore> = vec![NodeWithScore {
        score: 0,
        node: puzzle.map.clone(),
    }];
    // cameFrom
    let mut best_path_to_node: HashMap<Node, Node> = HashMap::new();
    // gScore
    let mut best_score_to_node: HashMap<Node, i32> = HashMap::new();
    best_score_to_node.insert(puzzle.map.clone(), 0);
    // fScore
    let mut best_guess_to_node: HashMap<Node, i32> = HashMap::new();
    best_guess_to_node.insert(puzzle.map.clone(), heuristic(&puzzle.map, goal));

    // Iterate on each cells
    while !open_set.is_empty() {
        let best_index = best_current_node(&open_set);
        let current = open_set.remove(best_index);

        // Check if it's the goal
        if current.node == *goal {
            return Ok(reconstruct_path(&best_path_to_node, &current.node));
        }

        for neighbor in neighbors(puzzle.size, &current.node).into_iter().flatten() {
            // println!("# checking {:#?}", neighbor);
            let goal_distance_for_neighbor = best_score_to_node.get(&current.node).unwrap() + 1;
            let neighbor_score = best_score_to_node.get(&neighbor);
            // println!("# checking {:#?}", neighbor);
            if neighbor_score.is_none() || goal_distance_for_neighbor < *neighbor_score.unwrap() {
                best_path_to_node.insert(neighbor.clone(), current.node.clone());
                best_score_to_node
                    .entry(neighbor.clone())
                    .or_insert(goal_distance_for_neighbor);
                let neighbor_distance_to_goal =
                    goal_distance_for_neighbor + heuristic(&neighbor, goal);
                best_guess_to_node
                    .entry(neighbor.clone())
                    .or_insert(neighbor_distance_to_goal);
                if !set_has_node(&open_set, &neighbor) {
                    let index = set_best_position(&open_set, neighbor_distance_to_goal);
                    open_set.insert(
                        index,
                        NodeWithScore {
                            score: neighbor_distance_to_goal,
                            node: neighbor.clone(),
                        },
                    );
                }
            }
        }
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
