use std::process;

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
        println!("{:#?}", puzzle);
        let goal = goal::generate(puzzle.size).unwrap_or_else(|err| {
            eprintln!("#> `{}`: {}", puzzle_path, err);
            process::exit(1);
        });
        println!("Goal {:#?}", goal);
        a_star::solve(&puzzle, &goal, heuristic::manhattan);
    }
}
