use std::env;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub variant: String,
    pub heuristic_name: String,
    pub solution_type: String,
    pub mode: String,
    pub files: Vec<String>,
    pub solvable: bool,
    pub amount: u32,
    pub size: i32,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let args: Vec<String> = env::args().skip(1).collect();

        // Parse each arguments as option or puzzle path
        let mut config = Config {
            variant: "ida*".to_string(),
            heuristic_name: "linear-conflicts".to_string(),
            solution_type: "snail".to_string(),
            mode: "normal".to_string(),
            files: Vec::new(),
            solvable: true,
            amount: 1,
            size: 3,
        };
        let mut found_first_puzzle = false;
        for arg in args.iter() {
            if !found_first_puzzle && arg.starts_with("--") {
                let split = arg.split_once('=');
                if let Some((option_name, value)) = split {
                    if option_name == "--variant" {
                        config.variant = value.to_string();
                    } else if option_name == "--heuristic" {
                        config.heuristic_name = value.to_string();
                    } else if option_name == "--solution-type" {
                        config.solution_type = value.to_string();
                    } else if option_name == "--mode" {
                        config.mode = value.to_string();
                    } else if option_name == "--unsolvable" {
                        config.solvable = false;
                    } else if option_name == "--amount" {
                        let amount = value.to_string().parse();
                        if let Err(err) = amount {
                            return Err(format!("Invalid amount `{}`: {}", value, err));
                        }
                        config.amount = amount.unwrap();
                    } else if option_name == "--size" {
                        let size = value.to_string().parse();
                        if let Err(err) = size {
                            return Err(format!("Invalid size `{}`: {}", value, err));
                        }
                        let size = size.unwrap();
                        if size < 3 {
                            return Err(format!("Invalid amount {}, must be at least 3", value));
                        }
                        config.size = size;
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

        Ok(config)
    }

    pub fn check_and_explain(&self) {
        if ![String::from("ida*"), String::from("a*")].contains(&self.variant) {
            eprintln!("Unknown variant: {}", self.variant);
            process::exit(1);
        }
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
        println!("Variant:             {}", self.variant);
        println!("Heuristic:           {}", self.heuristic_name);
        println!("Solution type:       {}", self.solution_type);
        println!("Mode:                {}", self.mode);
        if self.files.is_empty() {
            println!("(Generate) Size:     {}", self.size);
            println!("(Generate) Amount:   {}", self.amount);
            println!("(Generate) Solvable: {}", self.solvable);
        }
        println!("###");
    }
}
