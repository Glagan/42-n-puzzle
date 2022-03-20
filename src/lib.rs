use std::cmp::Ordering;

pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

struct Border {
    max: i32,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

struct Cursor {
    x: i32,
    y: i32,
    direction: Direction,
    value: i32,
}

pub struct SnailIterator {
    size: i32,
    max: i32,
    cursor: Cursor,
    border: Border,
}

impl SnailIterator {
    pub fn new(size: i32) -> SnailIterator {
        SnailIterator {
            size,
            max: (size * size),
            cursor: Cursor {
                x: 0,
                y: 0,
                direction: Direction::Right,
                value: 1,
            },
            border: Border {
                max: size - 1,
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
        }
    }

    pub fn position(&self) -> usize {
        (self.cursor.x + (self.cursor.y * self.size)) as usize
    }
}

impl Iterator for SnailIterator {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.value < self.max {
            let res = Some((
                (self.cursor.x + (self.cursor.y * self.size)) as usize,
                self.cursor.value,
            ));
            // Move cursor
            self.cursor.value += 1;
            match self.cursor.direction {
                Direction::Right => {
                    if self.cursor.x == self.border.max - self.border.right {
                        self.cursor.direction = Direction::Down;
                        self.border.top += 1;
                    }
                }
                Direction::Down => {
                    if self.cursor.y == self.border.max - self.border.bottom {
                        self.cursor.direction = Direction::Left;
                        self.border.right += 1;
                    }
                }
                Direction::Left => {
                    if self.cursor.x == self.border.left {
                        self.cursor.direction = Direction::Up;
                        self.border.bottom += 1;
                    }
                }
                Direction::Up => {
                    if self.cursor.y == self.border.top {
                        self.cursor.direction = Direction::Right;
                        self.border.left += 1;
                    }
                }
            };
            // Update cell
            self.cursor.x = match self.cursor.direction {
                Direction::Right => self.cursor.x + 1,
                Direction::Left => self.cursor.x - 1,
                _ => self.cursor.x,
            };
            self.cursor.y = match self.cursor.direction {
                Direction::Down => self.cursor.y + 1,
                Direction::Up => self.cursor.y - 1,
                _ => self.cursor.y,
            };
            // Return initial state
            res
        } else {
            None
        }
    }
}

pub fn neighbors(size: i32, source: &[i32]) -> [Option<Vec<i32>>; 4] {
    let index: i32 = source.iter().position(|&cell| cell == 0).unwrap() as i32;
    [
        // Left
        {
            if index == 0 || index % size == 0 {
                None
            } else {
                let mut cpy = Vec::from(source);
                let index: usize = index.try_into().unwrap();
                cpy[index] = cpy[index - 1];
                cpy[index - 1] = 0;
                Some(cpy)
            }
        },
        // Right
        {
            if (index + 1) % size == 0 {
                None
            } else {
                let mut cpy = Vec::from(source);
                let index: usize = index.try_into().unwrap();
                cpy[index] = cpy[index + 1];
                cpy[index + 1] = 0;
                Some(cpy)
            }
        },
        // Down
        {
            if index >= (size * (size - 1)) {
                None
            } else {
                let size: usize = size.try_into().unwrap();
                let mut cpy = Vec::from(source);
                let index: usize = index.try_into().unwrap();
                cpy[index] = cpy[index + size];
                cpy[index + size] = 0;
                Some(cpy)
            }
        },
        // Up
        {
            if index < size {
                None
            } else {
                let size: usize = size.try_into().unwrap();
                let mut cpy = Vec::from(source);
                let index: usize = index.try_into().unwrap();
                cpy[index] = cpy[index - size];
                cpy[index - size] = 0;
                Some(cpy)
            }
        },
    ]
}

#[test]
fn first_cell() {
    let source = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let neighbors = neighbors(3, &source);
    assert_eq!(neighbors[0], None);
    assert_eq!(neighbors[1], Some(vec![1, 0, 2, 3, 4, 5, 6, 7, 8]));
    assert_eq!(neighbors[2], Some(vec![3, 1, 2, 0, 4, 5, 6, 7, 8]));
    assert_eq!(neighbors[3], None);
}

#[test]
fn first_line_center() {
    let source = vec![1, 0, 2, 3, 4, 5, 6, 7, 8];
    let neighbors = neighbors(3, &source);
    assert_eq!(neighbors[0], Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]));
    assert_eq!(neighbors[1], Some(vec![1, 2, 0, 3, 4, 5, 6, 7, 8]));
    assert_eq!(neighbors[2], Some(vec![1, 4, 2, 3, 0, 5, 6, 7, 8]));
    assert_eq!(neighbors[3], None);
}

#[test]
fn middle_line_right() {
    let source = vec![1, 4, 2, 3, 5, 0, 6, 7, 8];
    let neighbors = neighbors(3, &source);
    assert_eq!(neighbors[0], Some(vec![1, 4, 2, 3, 0, 5, 6, 7, 8]));
    assert_eq!(neighbors[1], None);
    assert_eq!(neighbors[2], Some(vec![1, 4, 2, 3, 5, 8, 6, 7, 0]));
    assert_eq!(neighbors[3], Some(vec![1, 4, 0, 3, 5, 2, 6, 7, 8]));
}

#[test]
fn last_line() {
    let source = vec![1, 4, 2, 3, 5, 6, 0, 7, 8];
    let neighbors = neighbors(3, &source);
    assert_eq!(neighbors[0], None);
    assert_eq!(neighbors[1], Some(vec![1, 4, 2, 3, 5, 6, 7, 0, 8]));
    assert_eq!(neighbors[2], None);
    assert_eq!(neighbors[3], Some(vec![1, 4, 2, 0, 5, 6, 3, 7, 8]));
}

#[test]
fn last_line_first_column() {
    let source = vec![6, 7, 5, 4, 1, 8, 0, 2, 3];
    let neighbors = neighbors(3, &source);
    assert_eq!(neighbors[0], None);
    assert_eq!(neighbors[1], Some(vec![6, 7, 5, 4, 1, 8, 2, 0, 3]));
    assert_eq!(neighbors[2], None);
    assert_eq!(neighbors[3], Some(vec![6, 7, 5, 0, 1, 8, 4, 2, 3]));
}

pub enum Mode {
    Normal,
    Greedy,
    Uniform,
}

#[derive(Clone)]
pub struct NodeWithCost {
    pub cost: f64,
    pub node: Vec<i32>,
}

impl Eq for NodeWithCost {}

impl PartialEq for NodeWithCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap().reverse()
    }
}

impl PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}

pub struct Solution {
    pub total_used_states: i32,
    pub biggest_state: usize,
    pub steps: Vec<Vec<i32>>,
}

pub fn print_map(size: i32, map: &[i32]) {
    let size: usize = size.try_into().unwrap();
    for (index, value) in map.iter().enumerate() {
        if *value == 0 {
            print!("    ");
        } else {
            print!("{:3} ", value);
        }
        if (index + 1) % size == 0 {
            println!();
        }
    }
}
