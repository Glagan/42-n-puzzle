use std::process;
use std::time::Instant;

use npuzzle::print_map;

mod a_star;
mod config;
mod goal;
mod heuristic;
mod puzzle;

fn main() {
    let config = config::Config::new().unwrap_or_else(|err| {
        eprintln!("Failed to generate config: {}", err);
        process::exit(1);
    });
    for puzzle_path in config.files {
        println!("# Puzzle {}", puzzle_path);
        let puzzle = puzzle::Puzzle::new(&puzzle_path).unwrap_or_else(|err| {
            eprintln!("#> `{}`: {}", puzzle_path, err);
            process::exit(1);
        });
        println!("{}", puzzle);
        let goal = goal::generate(puzzle.size).unwrap_or_else(|err| {
            eprintln!("#> `{}`: {}", puzzle_path, err);
            process::exit(1);
        });
        print_map(puzzle.size, &goal);
        let now = Instant::now();
        let res = a_star::solve(&puzzle, &goal, heuristic::manhattan);
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
                    println!("{:3} {}", "#".repeat((index % size) + 1), index);
                    print_map(puzzle.size, step);
                }
            }
            Err(_) => eprintln!("#> No solution found in {:.2?}", elapsed),
        }
    }
}
