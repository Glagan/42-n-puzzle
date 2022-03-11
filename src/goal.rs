enum Direction {
    Left,
    Right,
    Down,
    Up,
}

struct Border {
    max: u32,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

struct Cursor {
    x: u32,
    y: u32,
    direction: Direction,
    value: u32,
}

pub fn generate(size: u32) -> Result<Vec<u32>, String> {
    if size < 3 {
        return Err(format!("Invalid size {}, must be at least 3", size));
    }
    let puzzle_size = size * size;
    let mut solution: Vec<u32> = (1..=puzzle_size).collect();
    let mut border = Border {
        max: size - 1,
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    let mut cursor = Cursor {
        x: 0,
        y: 0,
        direction: Direction::Right,
        value: 1,
    };
    // Iterate for each cells to add each numbers in "snail" order
    for _ in 0..puzzle_size - 1 {
        println!("cell ({},{}) = {}", cursor.x, cursor.y, cursor.value);
        let v: &mut u32 = &mut solution[(cursor.x + (cursor.y * size)) as usize];
        *v = cursor.value;
        cursor.value += 1;
        // Update direction
        match cursor.direction {
            Direction::Right => {
                if cursor.x == border.max - border.right {
                    cursor.direction = Direction::Down;
                    border.top += 1;
                }
            }
            Direction::Down => {
                if cursor.y == border.max - border.bottom {
                    cursor.direction = Direction::Left;
                    border.right += 1;
                }
            }
            Direction::Left => {
                if cursor.x == 0 + border.left {
                    cursor.direction = Direction::Up;
                    border.bottom += 1;
                }
            }
            Direction::Up => {
                if cursor.y == 0 + border.top {
                    cursor.direction = Direction::Right;
                    border.left += 1;
                }
            }
        };
        // Update cell
        cursor.x = match cursor.direction {
            Direction::Right => cursor.x + 1,
            Direction::Left => cursor.x - 1,
            _ => cursor.x,
        };
        cursor.y = match cursor.direction {
            Direction::Down => cursor.y + 1,
            Direction::Up => cursor.y - 1,
            _ => cursor.y,
        };
    }
    // Last empty cell
    let v: &mut u32 = &mut solution[(cursor.x + (cursor.y * size)) as usize];
    *v = 0;
    Ok(solution)
}

#[test]
fn test_generate_goal_2() {
    let solution = generate(2);
    assert_eq!(true, solution.is_err())
}

#[test]
fn test_generate_goal_3() {
    let solution = generate(3);
    assert_eq!(solution, Ok(vec![1, 2, 3, 8, 0, 4, 7, 6, 5]))
}

#[test]
fn test_generate_goal_4() {
    let solution = generate(4);
    assert_eq!(
        solution,
        Ok(vec![1, 2, 3, 4, 12, 13, 14, 5, 11, 0, 15, 6, 10, 9, 8, 7])
    )
}

#[test]
fn test_generate_goal_5() {
    let solution = generate(5);
    assert_eq!(
        solution,
        Ok(vec![
            1, 2, 3, 4, 5, 16, 17, 18, 19, 6, 15, 24, 0, 20, 7, 14, 23, 22, 21, 8, 13, 12, 11, 10,
            9
        ])
    )
}