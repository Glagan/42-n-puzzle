use std::env;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub heuristic_name: String,
    pub solution_type: String,
    pub mode: String,
    pub files: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.is_empty() {
            return Err("You must add at least one argument as the puzzle to solve".to_string());
        }

        // Parse each arguments as option or puzzle path
        let mut config = Config {
            heuristic_name: "linear-conflict".to_string(),
            solution_type: "snail".to_string(),
            mode: "normal".to_string(),
            files: Vec::new(),
        };
        let mut found_first_puzzle = false;
        for arg in args.iter() {
            if !found_first_puzzle && arg.starts_with("--") {
                let split = arg.split_once('=');
                if let Some((option_name, value)) = split {
                    if option_name == "--heuristic" {
                        config.heuristic_name = value.to_string();
                    } else if option_name == "--solution-type" {
                        config.solution_type = value.to_string();
                    } else if option_name == "--mode" {
                        config.mode = value.to_string();
                    }
                } else {
                    return Err(format!("Malformed argument {}", arg));
                }
            } else {
                found_first_puzzle = true;
            }
            if found_first_puzzle {
                config.files.push(arg.clone());
            }
        }

        // Check config values
        if config.files.is_empty() {
            return Err("You must add at least one puzzle to solve".to_string());
        }

        Ok(config)
    }

    pub fn check_and_explain(&self) {
        if ![
            String::from("snail"),
            String::from("first"),
            String::from("last"),
        ]
        .contains(&self.solution_type)
        {
            eprintln!("Unknown solution type: {}", self.solution_type);
            process::exit(1);
        }
        if ![
            String::from("normal"),
            String::from("greedy"),
            String::from("uniform"),
        ]
        .contains(&self.mode)
        {
            eprintln!("Unknown mode: {}", self.mode);
            process::exit(1);
        }
        println!("###");
        println!("Using heuristic: {}", self.heuristic_name);
        println!("Solution type:   {}", self.solution_type);
        println!("Mode:            {}", self.mode);
        println!("###");
    }
}
