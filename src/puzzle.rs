use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Puzzle {
    size: u32,
    map: Vec<u32>,
}

impl Puzzle {
    fn parse_line(line: &str) -> Result<Option<Vec<u32>>, String> {
        let mut line = line.trim();
        if line.len() == 0 {
            return Ok(None);
        }
        // Remove comment
        let comment_start = line.find('#');
        if let Some(byte) = comment_start {
            if byte == 0 {
                return Ok(None);
            }
            line = (&line[0..byte]).trim()
        }
        // Parse each cols
        let cols: Vec<Result<u32, ParseIntError>> = line
            .split_whitespace()
            .filter(|col| col.len() > 0)
            .map(|col| col.trim().parse())
            .collect();
        let mut clean_cols: Vec<u32> = Vec::new();
        for col in cols {
            if col.is_err() {
                return Err(format!("Failed to parse line `{}`: {:#?}", line, col));
            }
            clean_cols.push(col.unwrap())
        }
        return Ok(Some(clean_cols));
    }

    fn parse_content(content: &str) -> Result<Puzzle, String> {
        let mut size: u32 = 0;
        let mut empty_col: bool = false;
        let mut map: Vec<u32> = Vec::new();
        // Parse each lines and check for errors
        for line in content.lines() {
            let parsed_line = Puzzle::parse_line(line)?;
            if let None = parsed_line {
                continue;
            } else if let Some(cols) = parsed_line {
                if size == 0 {
                    if cols.len() != 1 {
                        return Err(
                            "Expected only 1 value as the puzzle size on the first line."
                                .to_string(),
                        );
                    }
                    size = cols[0];
                } else {
                    if cols.len() != size.try_into().unwrap() {
                        return Err(format!(
                            "Invalid number of cells for the line `{}`, expected {}.",
                            line, size,
                        ));
                    }
                    for &col in cols.iter() {
                        if col == 0 {
                            if empty_col {
                                return Err(
                                    "There can be only one empty cell in the puzzle.".to_string()
                                );
                            } else {
                                empty_col = true;
                            }
                        }
                        map.push(col)
                    }
                }
            }
        }
        // Check final size, in case of missing or extra lines
        let cell_count: u32 = map.len().try_into().unwrap();
        let expected_count = size * size;
        if cell_count != expected_count {
            return Err(format!(
                "Invalid number of cells `{}`, expected {}.",
                cell_count, expected_count
            ));
        }
        return Ok(Puzzle { size, map });
    }

    pub fn new(path: &str) -> Result<Puzzle, String> {
        let content = fs::read_to_string(path);
        if let Err(e) = content {
            return Err(format!("Failed to open or read puzzle file: {}", e));
        }
        let content = content.unwrap();
        Puzzle::parse_content(&content)
    }
}
