// Number of different cells between two Nodes
pub fn hamming(_: i32, node: &[i32], goal: &[i32]) -> f64 {
    node.iter()
        .zip(goal)
        .map(|(x, y)| if *x > 0 && x != y { 1. } else { 0. })
        .sum()
}

#[test]
fn hamming_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(hamming(3, &left, &right), 1.)
}

#[test]
fn hamming_two() {
    let left = vec![18, 8, 7, 4, 9, 6, 12, 24, 11];
    let right = vec![12, 24, 18, 8, 7, 4, 11, 9, 6];
    assert_eq!(hamming(3, &left, &right), 9.)
}

pub fn manhattan_distance(size: i32, index: usize, goal: usize) -> f64 {
    let size = size as usize;
    let (x_n, y_n) = ((index % size) as i32, (index / size) as i32);
    let (x_g, y_g) = ((goal % size) as i32, (goal / size) as i32);
    (x_n - x_g).abs() as f64 + (y_n - y_g).abs() as f64
}

// Sum of the manhattan distance for each cell in the Node
// sum(abs(x - y))
pub fn manhattan(size: i32, node: &[i32], goal: &[i32]) -> f64 {
    node.iter()
        .zip(goal)
        .enumerate()
        .map(|(index, (&x, &y))| {
            if x > 0 && x != y {
                let goal_index = goal.iter().position(|&to_find| x == to_find).unwrap();
                manhattan_distance(size, index, goal_index)
            } else {
                0.
            }
        })
        .sum::<f64>()
}

#[test]
fn manhattan_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(manhattan(3, &left, &right), 1.)
}

#[test]
fn manhattan_two() {
    let left = vec![18, 8, 7, 4, 9, 6, 12, 24, 11];
    let right = vec![12, 24, 18, 8, 7, 4, 11, 9, 6];
    assert_eq!(manhattan(3, &left, &right), 16.)
}

#[test]
fn manhattan_three() {
    let left = vec![7, 2, 4, 5, 0, 6, 8, 3, 1];
    let right = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
    assert_eq!(manhattan(3, &left, &right), 16.)
}

// Sum of the euclidean distance for each cell in the Node
// sqrt(sum((x - y) ** 2))
pub fn euclidean_distance(size: i32, node: &[i32], goal: &[i32]) -> f64 {
    node.iter()
        .zip(goal)
        .enumerate()
        .map(|(index, (&x, &y))| {
            if x > 0 && x != y {
                let size = size as usize;
                let goal_index = goal.iter().position(|&to_find| x == to_find).unwrap();
                let (x_n, y_n) = ((index % size) as i32, (index / size) as i32);
                let (x_g, y_g) = ((goal_index % size) as i32, (goal_index / size) as i32);
                ((x_n - x_g) as f64).powf(2.) + ((y_n - y_g) as f64).powf(2.)
            } else {
                0.
            }
        })
        .sum::<f64>()
        .sqrt()
}

#[test]
fn euclidean_distance_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(euclidean_distance(3, &left, &right), 1.)
}

#[test]
fn euclidean_distance_two() {
    let left = vec![18, 8, 7, 4, 9, 6, 12, 24, 11];
    let right = vec![12, 24, 18, 8, 7, 4, 11, 9, 6];
    assert_eq!(euclidean_distance(3, &left, &right), 5.0990195135927845)
}

struct LinearConflictCell {
    goal: usize,
    goal_line: usize,
    goal_column: usize,
    line: usize,
    column: usize,
}

// Sum of the manhattan distance + linear conflicts for each cell in the Node
// sum(abs(x - y)) + 2*linear_conflicts
pub fn linear_conflicts(size: i32, node: &[i32], goal: &[i32]) -> f64 {
    let manhattan_distance = manhattan(size, node, goal);
    let size: usize = size.try_into().unwrap();
    let mut linear_conflicts = 0.;

    // Build goal map to avoid recalculation
    let cell_state: Vec<LinearConflictCell> = (0..(size * size))
        .map(|index| {
            let cell_value = node[index];
            let goal = goal.iter().position(|&v| cell_value == v).unwrap();
            LinearConflictCell {
                goal,
                goal_line: goal / size,
                goal_column: goal % size,
                line: index / size,
                column: index % size,
            }
        })
        .collect();

    // j is the node being checked and k is the next in row/column
    for (index, &j) in node.iter().enumerate() {
        // Ignore empty cell
        if j == 0 {
            continue;
        }
        // Calculate j shared state between rows and columns
        let current_cell = &cell_state[index];
        // Process cells on the same line for horizontal conflicts
        if current_cell.column > 0 && current_cell.line == current_cell.goal_line {
            // j is on the right, k is on the left
            for offset in 1..=current_cell.column {
                let k = node[index - offset];
                if k == 0 {
                    continue;
                }
                let k_cell = &cell_state[index - offset];
                // The goal positions of j and k are on the same lines
                // -- and j is on the right from k
                if current_cell.goal_line == k_cell.goal_line && k_cell.goal >= current_cell.goal {
                    linear_conflicts += 1.;
                }
            }
        }
        // --  and row for vertical conflicts
        if current_cell.line > 0 && current_cell.column == current_cell.goal_column {
            // j is down, k is on top
            for offset in 1..=current_cell.line {
                let k = node[index - (offset * size)];
                if k == 0 {
                    continue;
                }
                let k_cell = &cell_state[index - (offset * size)];
                // The goal positions of j and k are on the same columns
                // -- and j is down from k
                if current_cell.goal_column == k_cell.goal_column
                    && k_cell.goal >= current_cell.goal
                {
                    linear_conflicts += 1.;
                }
            }
        }
    }

    manhattan_distance + (linear_conflicts * 2.)
}

#[test]
fn linear_conflicts_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(linear_conflicts(3, &left, &right), 1.)
}

#[test]
fn linear_conflicts_two() {
    let left = vec![18, 8, 7, 4, 9, 6, 12, 24, 11];
    let right = vec![12, 24, 18, 8, 7, 4, 11, 9, 6];
    assert_eq!(linear_conflicts(3, &left, &right), 16.)
}

#[test]
fn linear_conflicts_three() {
    let left = vec![2, 1, 3, 8, 0, 4, 7, 6, 5];
    let right = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
    assert_eq!(linear_conflicts(3, &left, &right), 4.)
}

#[test]
fn linear_conflicts_four() {
    let left = vec![8, 2, 3, 1, 0, 4, 7, 6, 5];
    let right = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
    assert_eq!(linear_conflicts(3, &left, &right), 4.)
}
