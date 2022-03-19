use std::process;
use std::time::Instant;

use npuzzle::{print_map, Node};
use puzzle::Puzzle;

mod a_star;
mod config;
mod goal;
mod heuristic;
mod puzzle;

fn heuristic_by_name(name: &str) -> Option<fn(i32, &Node, &Node) -> f64> {
    if name == "manhattan" {
        return Some(heuristic::manhattan);
    } else if name == "euclidean" {
        return Some(heuristic::euclidean_distance);
    } else if name == "hamming" {
        return Some(heuristic::hamming);
    } else if name == "linear-conflict" {
        return Some(heuristic::linear_conflict);
    }
    None
}

fn solve_puzzle(
    config: &config::Config,
    puzzle: &Puzzle,
    heuristic_fn: fn(i32, &Node, &Node) -> f64,
) {
    println!("{}", puzzle);
    print_map(puzzle.size, &puzzle.goal);

    if !puzzle.is_solvable() {
        println!("#> Puzzle is unsolvable for this solution");
        return;
    }

    let now = Instant::now();
    let res = a_star::solve(puzzle, &config.mode, heuristic_fn);
    let elapsed = now.elapsed();
    match res {
        // Ok(solution) => println!("#> Solution {:#?}", solution),
        Ok(solution) => {
            println!(
                "#> Solution found ({:#?} {}) in {:.2?}",
                solution.steps.len() - 1,
                if solution.steps.len() > 2 {
                    "steps"
                } else {
                    "step"
                },
                elapsed,
            );
            println!(
                "#> Total number of states ever selected: {}",
                solution.total_used_states
            );
            println!(
                "#> Maximum number of states ever represented in memory: {}",
                solution.biggest_state
            );
            let size: usize = puzzle.size.try_into().unwrap();
            for (index, step) in solution.steps.iter().enumerate() {
                println!("{:3} {}", index, "#".repeat((index % size) + 1));
                print_map(puzzle.size, step);
            }
        }
        Err(_) => eprintln!("#> No solution found in {:.2?}", elapsed),
    }
}

fn main() {
    let config = config::Config::new().unwrap_or_else(|err| {
        eprintln!("Failed to generate config: {}", err);
        process::exit(1);
    });

    // Check config values
    let heuristic_fn = heuristic_by_name(&config.heuristic_name).unwrap_or_else(|| {
        eprintln!("Unknown heuristic: {}", config.heuristic_name);
        process::exit(1);
    });
    config.check_and_explain();

    //  Solve each puzzles
    if config.files.is_empty() {
        println!("# Random Puzzle");
        let puzzle = puzzle::Puzzle::generate(3, &config.solution_type).unwrap_or_else(|err| {
            eprintln!("#> {}", err);
            process::exit(1);
        });
        solve_puzzle(&config, &puzzle, heuristic_fn);
    } else {
        for puzzle_path in &config.files {
            println!("# Puzzle {}", puzzle_path);
            let puzzle =
                puzzle::Puzzle::new(puzzle_path, &config.solution_type).unwrap_or_else(|err| {
                    eprintln!("#> {}", err);
                    process::exit(1);
                });
            solve_puzzle(&config, &puzzle, heuristic_fn);
        }
    }
}
