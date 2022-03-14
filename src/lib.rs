use std::slice::Iter;

pub type Node = Vec<i32>;

pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Left,
            Direction::Right,
            Direction::Down,
            Direction::Up,
        ];
        DIRECTIONS.iter()
    }
}

pub fn neighbors(size: i32, source: &Node) -> [Option<Node>; 4] {
    let index: i32 = source.iter().position(|&cell| cell == 0).unwrap() as i32;
    [
        // Left
        {
            if index == 0 || index % size == 0 {
                None
            } else {
                let mut cpy = source.clone();
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
                let mut cpy = source.clone();
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
                let mut cpy = source.clone();
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
                let mut cpy = source.clone();
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

pub fn print_map(size: i32, map: &Node) {
    println!("{}", size);
    let size: usize = size.try_into().unwrap();
    for (index, value) in map.iter().enumerate() {
        print!("{:3} ", value);
        if (index + 1) % size == 0 {
            println!();
        }
    }
}
