use npuzzle::{print_map, HeuristicFn, Solution};
use puzzle::Puzzle;
use std::process;
use std::time::Instant;

mod a_star;
mod config;
mod goal;
mod heuristic;
mod ida_star;
mod puzzle;

type SolveFn = fn(&Puzzle, &str, fn(i32, &[i32], &[i32]) -> f64) -> Result<Solution, String>;

fn solve_by_name(name: &str) -> Option<SolveFn> {
    if name == "ida*" {
        return Some(ida_star::solve);
    } else if name == "a*" {
        return Some(a_star::solve);
    }
    None
}

fn heuristic_by_name(name: &str) -> Option<HeuristicFn> {
    if name == "manhattan" {
        return Some(heuristic::manhattan);
    } else if name == "euclidean" {
        return Some(heuristic::euclidean_distance);
    } else if name == "hamming" {
        return Some(heuristic::hamming);
    } else if name == "linear-conflicts" {
        return Some(heuristic::linear_conflicts);
    }
    None
}

fn solve_puzzle(
    config: &config::Config,
    puzzle: &Puzzle,
    solve_fn: SolveFn,
    heuristic_fn: HeuristicFn,
) {
    println!("{}", puzzle);
    print_map(puzzle.size, &puzzle.goal);

    if !Puzzle::is_solvable(puzzle) {
        println!("#> Puzzle is unsolvable for this solution");
        return;
    }

    let now = Instant::now();
    let res = solve_fn(puzzle, &config.mode, heuristic_fn);
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
                println!("{:<3} {}", index, "#".repeat((index % size) + 1));
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

    // Select variant
    let solve_fn = solve_by_name(&config.variant).unwrap_or_else(|| {
        eprintln!("Unknown variant: {}", config.variant);
        process::exit(1);
    });

    //  Solve each puzzles
    let now = Instant::now();
    if config.files.is_empty() {
        if config.variant == "ida*" && config.mode == "greedy" && config.size > 4 {
            println!(
                "> Greedy mode can't be used with IDA* for puzzle larger than 4 (Stack Overflow)"
            );
            return;
        }
        for i in 1..=config.amount {
            println!("# Random Puzzle [{}]", i);
            let puzzle = Puzzle::generate(config.solvable, config.size, &config.solution_type);
            if let Err(err) = puzzle {
                eprintln!("#> {}", err);
            } else {
                solve_puzzle(&config, &puzzle.unwrap(), solve_fn, heuristic_fn);
            }
        }
    } else {
        for puzzle_path in &config.files {
            println!("# {}", puzzle_path);
            let puzzle = Puzzle::new(puzzle_path, &config.solution_type);
            if let Err(err) = puzzle {
                eprintln!("#> {}", err);
            } else {
                let puzzle = puzzle.unwrap();
                if config.variant == "ida*" && config.mode == "greedy" && puzzle.size > 4 {
                    println!("#> Greedy mode can't be used with IDA* for puzzle larger than 4 (Stack Overflow)");
                    return;
                }
                solve_puzzle(&config, &puzzle, solve_fn, heuristic_fn);
            }
        }
    }

    // Total duration if there was multiple puzzles
    if (config.files.is_empty() && config.amount > 1) || config.files.len() > 1 {
        println!(
            "> Solved {} puzzles in {:.2?}",
            if config.files.is_empty() {
                config.amount
            } else {
                config.files.len() as u32
            },
            now.elapsed()
        );
    }
}
