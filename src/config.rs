use std::env;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let files: Vec<String> = env::args().skip(1).collect();
        if !files.is_empty() {
            return Err("You must add at least one puzzle");
        }
        Ok(Config { files })
    }
}
