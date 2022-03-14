use crate::puzzle::Puzzle;
use npuzzle::{neighbors, Node};
use std::collections::HashMap;

pub fn solve(
    puzzle: &Puzzle,
    goal: &Node,
    heuristic: fn(&Node, &Node) -> i32,
) -> Result<Vec<Node>, String> {
    let start = &puzzle.map;
    let mut open: Vec<&Node> = vec![start];
    let mut path: Vec<&Node> = Vec::new();
    let mut current_score: HashMap<&Node, i32> = HashMap::new();
    current_score.insert(start, 0);
    let mut best_guess: HashMap<&Node, i32> = HashMap::new();
    best_guess.insert(start, heuristic(start, goal));

    while !open.is_empty() {
        let current = open.pop().unwrap();
        if current == goal {
            // TODO reconstruct_path
            return Ok(Vec::new());
        }

        for &neighbor in neighbors(puzzle.size, current) {
            let neighbor_score = current_score[current] + 1;
            if !current_score.contains_key(&neighbor)
                || neighbor_score < *(current_score.get(&neighbor).unwrap())
            {
                path.push(current);
                current_score.insert(&neighbor, neighbor_score);
                best_guess.insert(&neighbor, neighbor_score + heuristic(&neighbor, goal));
                if !open.contains(&neighbor) {
                    open.push(&neighbor);
                }
            }
        }
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
