use crate::{puzzle::Puzzle, HeuristicFn};
use npuzzle::{neighbors, Mode, Node, Solution};
use std::time::Instant;

pub struct Summary {
    total_used_states: i32,
    biggest_state: usize,
}

pub struct Branch<'a> {
    path: &'a mut Vec<Node>,
    depth: f64,
    bound: f64,
}

pub struct BranchResult {
    score: f64,
    result: Option<Node>,
}

pub fn evaluate_branch(
    puzzle: &Puzzle,
    summary: &mut Summary,
    branch: &mut Branch,
    heuristic: HeuristicFn,
    mode: &Mode,
) -> BranchResult {
    summary.total_used_states += 1;
    let node = branch.path.last().unwrap();
    // Check if node is withinn bound
    let f = match mode {
        Mode::Normal => branch.depth + heuristic(puzzle.size, node, &puzzle.goal),
        Mode::Greedy => heuristic(puzzle.size, node, &puzzle.goal), // Ignore depth
        Mode::Uniform => branch.depth,                              // Ignore heuristic
    };
    if f > branch.bound {
        return BranchResult {
            score: f,
            result: None,
        };
    }
    if *node == puzzle.goal {
        return BranchResult {
            score: f,
            result: Some(node.clone()),
        };
    }
    // Check each neighors
    let mut min = f64::INFINITY;
    for neighbor in neighbors(puzzle.size, node).into_iter().flatten() {
        if branch.path.contains(&neighbor) {
            continue;
        }
        branch.path.push(neighbor);
        if branch.path.len() > summary.biggest_state {
            summary.biggest_state = branch.path.len();
        }
        let branch_result = evaluate_branch(
            puzzle,
            summary,
            &mut Branch {
                path: branch.path,
                depth: branch.depth + 1.,
                bound: branch.bound,
            },
            heuristic,
            mode,
        );
        if branch_result.result.is_some() {
            return branch_result;
        }
        if branch_result.score < min {
            min = branch_result.score
        }
        branch.path.pop();
    }
    BranchResult {
        score: min,
        result: None,
    }
}

pub fn solve(puzzle: &Puzzle, mode: &str, heuristic: HeuristicFn) -> Result<Solution, String> {
    let now = Instant::now();
    let mode = match mode {
        "greedy" => Mode::Greedy,
        "uniform" => Mode::Uniform,
        _ => Mode::Normal,
    };

    // State
    let mut summary = Summary {
        total_used_states: 0,
        biggest_state: 1, // 1 is the initial state
    };
    let mut bound = match mode {
        Mode::Uniform => 1., // Ignore heuristic
        _ => heuristic(puzzle.size, &puzzle.map, &puzzle.goal),
    };
    let mut path = vec![puzzle.map.clone()];

    loop {
        let mut branch = Branch {
            path: &mut path,
            depth: 0.,
            bound,
        };
        let result = evaluate_branch(puzzle, &mut summary, &mut branch, heuristic, &mode);
        if result.result.is_some() {
            return Ok(Solution {
                biggest_state: summary.biggest_state,
                total_used_states: summary.total_used_states,
                steps: path,
            });
        }
        if result.score == f64::INFINITY {
            return Err(String::from("Failed to find a solution for this puzzle"));
        }
        println!(
            "#> Explored {} states to bound {} in {:.2?}",
            summary.total_used_states,
            bound,
            now.elapsed()
        );
        bound = result.score;
    }
}
