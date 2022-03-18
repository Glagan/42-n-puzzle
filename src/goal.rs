use npuzzle::{Node, SnailIterator};

pub fn generate_snail(size: i32) -> Result<Node, String> {
    let puzzle_size = size * size;
    let mut solution: Node = (1..=puzzle_size).collect();
    let mut iterator = SnailIterator::new(size);
    // Iterate for each cells to add each numbers in "snail" order
    for (index, value) in iterator.by_ref() {
        let v: &mut i32 = &mut solution[index];
        *v = value;
    }
    // Last empty cell
    let v: &mut i32 = &mut solution[iterator.position()];
    *v = 0;
    Ok(solution)
}

pub fn generate_first(size: i32) -> Result<Node, String> {
    let puzzle_size = size * size;
    let mut solution: Node = (1..puzzle_size).collect();
    solution.insert(0, 0);
    Ok(solution)
}

pub fn generate_last(size: i32) -> Result<Node, String> {
    let puzzle_size = size * size;
    let mut solution: Node = (1..puzzle_size).collect();
    solution.push(0);
    Ok(solution)
}

pub fn generate(size: i32, solution_type: &str) -> Result<Node, String> {
    if size < 3 {
        return Err(format!("Invalid size {}, must be at least 3", size));
    }

    if solution_type == "snail" {
        return generate_snail(size);
    } else if solution_type == "first" {
        return generate_first(size);
    } else if solution_type == "last" {
        return generate_last(size);
    }
    Err(format!("Unknown solution type `{}`", solution_type))
}

#[test]
fn snail_generate_goal_2() {
    let solution = generate(2, &String::from("snail"));
    assert!(solution.is_err())
}

#[test]
fn snail_generate_goal_3() {
    let solution = generate(3, &String::from("snail"));
    assert_eq!(solution, Ok(vec![1, 2, 3, 8, 0, 4, 7, 6, 5]))
}

#[test]
fn snail_generate_goal_4() {
    let solution = generate(4, &String::from("snail"));
    assert_eq!(
        solution,
        Ok(vec![1, 2, 3, 4, 12, 13, 14, 5, 11, 0, 15, 6, 10, 9, 8, 7])
    )
}

#[test]
fn snail_generate_goal_5() {
    let solution = generate(5, &String::from("snail"));
    assert_eq!(
        solution,
        Ok(vec![
            1, 2, 3, 4, 5, 16, 17, 18, 19, 6, 15, 24, 0, 20, 7, 14, 23, 22, 21, 8, 13, 12, 11, 10,
            9
        ])
    )
}
