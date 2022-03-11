use npuzzle::Config;
use npuzzle::Puzzle;
use std::process;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Failed to generate config: {}", err);
        process::exit(1);
    });
    for puzzle_path in config.files {
        println!("# Puzzle {}", puzzle_path);
        let puzzle = Puzzle::new(&puzzle_path).unwrap_or_else(|err| {
            eprintln!("#> `{}`: {}", puzzle_path, err);
            process::exit(1);
        });
        println!("{:#?}", puzzle);
    }
}
