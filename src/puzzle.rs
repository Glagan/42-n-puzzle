use crate::goal;
use core::fmt;
use npuzzle::{neighbors, Node};
use std::fs;
use std::num::ParseIntError;

pub struct Puzzle {
    pub size: i32,
    pub map: Node,
    pub goal: Node,
}

impl Puzzle {
    fn parse_line(line: &str) -> Result<Option<Vec<i32>>, String> {
        let mut line = line.trim();
        if line.is_empty() {
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
        let cols: Vec<Result<i32, ParseIntError>> = line
            .split_whitespace()
            .filter(|col| !col.is_empty())
            .map(|col| col.trim().parse())
            .collect();
        let mut clean_cols: Vec<i32> = Vec::new();
        for col in cols {
            if col.is_err() {
                return Err(format!("Failed to parse line `{}`: {:#?}", line, col));
            }
            clean_cols.push(col.unwrap())
        }
        Ok(Some(clean_cols))
    }

    fn parse_content(content: &str) -> Result<(i32, Node), String> {
        let mut size: i32 = 0;
        let mut empty_col: bool = false;
        let mut map: Node = Vec::new();

        // Parse each lines and check for errors
        for line in content.lines() {
            let parsed_line = Puzzle::parse_line(line)?;
            if parsed_line.is_none() {
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
        let cell_count: i32 = map.len().try_into().unwrap();
        let expected_count = size * size;
        if cell_count != expected_count {
            return Err(format!(
                "Invalid number of cells `{}`, expected {}.",
                cell_count, expected_count
            ));
        }

        Ok((size, map))
    }

    pub fn new(path: &str, solution_type: &str) -> Result<Puzzle, String> {
        let content = fs::read_to_string(path);
        if let Err(e) = content {
            return Err(format!("Failed to open or read puzzle file: {}", e));
        }

        let content = content.unwrap();
        let (size, map) = Puzzle::parse_content(&content)?;

        let map_goal = goal::generate(size, solution_type)?;
        Ok(Puzzle {
            size,
            map,
            goal: map_goal,
        })
    }

    pub fn neighbors(&self, node: &Node) -> [Option<Node>; 4] {
        neighbors(self.size, node)
    }

    pub fn is_solvable(&self) -> bool {
        let size: usize = self.size.try_into().unwrap();
        // Count number of movements and the empty row depending on each solutions
        let empty_row: usize = (self.map.iter().position(|&v| v == 0).unwrap() / size) + 1;
        let mut inversions = 0;
        for (index, &i) in self.map.iter().enumerate() {
            for &j in self.map.iter().skip(index) {
                if j != 0 && i > j {
                    inversions += 1;
                }
            }
        }
        // Even puzzle size
        if size % 2 == 0 {
            // Row with empty cell is even
            if empty_row % 2 == 0 {
                inversions % 2 != 0
            } else {
                inversions % 2 == 0
            }
        } else {
            inversions % 2 == 0
        }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = fmt::Result::Ok(());
        let size: usize = self.size.try_into().unwrap();
        for (index, value) in self.map.iter().enumerate() {
            if *value == 0 {
                res = write!(f, "    ");
            } else {
                res = write!(f, "{:3} ", value);
            }
            if (index + 1) % size == 0 {
                res = writeln!(f);
            }
        }
        res
    }
}
