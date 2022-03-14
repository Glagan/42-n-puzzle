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
            Ok(solution) => println!(
                "#> Solution found ({:#?} steps) in {:.2?}",
                solution.len(),
                elapsed
            ),
            Err(_) => eprintln!("#> No solution found in {:.2?}", elapsed),
        }
    }
}
