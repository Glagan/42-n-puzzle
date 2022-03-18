use npuzzle::Node;

// Number of different cells between two Nodes
pub fn hamming(_: i32, node: &Node, goal: &Node) -> f64 {
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

fn manhattan_distance(size: i32, index: usize, goal: usize) -> f64 {
    let x_n = (index % size as usize) as i32;
    let y_n = (index as f64 / 3.).floor() as i32;
    let x_g = (goal % size as usize) as i32;
    let y_g = (goal as f64 / 3.).floor() as i32;
    (x_n - x_g).abs() as f64 + (y_n - y_g).abs() as f64
}

// Sum of the manhattan distance for each cell in the Node
// sum(abs(x - y))
pub fn manhattan(size: i32, node: &Node, goal: &Node) -> f64 {
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
pub fn euclidean_distance(size: i32, node: &Node, goal: &Node) -> f64 {
    node.iter()
        .zip(goal)
        .enumerate()
        .map(|(index, (&x, &y))| {
            if x > 0 && x != y {
                let x_n = (index % size as usize) as i32;
                let y_n = (index as f64 / 3.).floor() as i32;
                let index = goal.iter().position(|&to_find| x == to_find).unwrap();
                let x_g = (index % size as usize) as i32;
                let y_g = (index as f64 / 3.).floor() as i32;
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

// Sum of the manhattan distance + linear conflicts for each cell in the Node
// sum(abs(x - y)) + 2*linear_conflicts
pub fn linear_conflict(size: i32, node: &Node, goal: &Node) -> f64 {
    let manhattan_distance = manhattan(size, node, goal);
    let size: usize = size.try_into().unwrap();
    let mut linear_conflicts = 0.;

    // j is the node being checked and k is the next in row/column
    let total_len = node.len();
    for (index, &j) in node.iter().enumerate() {
        if index + 1 == node.len() {
            break;
        }
        // Process cells on the same line for horizontal conflicts
        if (index + 1) % size != 0 {
            // j is on the left, k is on the right
            let k = node[index + 1];
            // Ignore empty cell
            if j == 0 || k == 0 {
                continue;
            }
            let line = index / size;
            let goal_j = goal.iter().position(|&v| j == v).unwrap();
            let goal_k = goal.iter().position(|&v| k == v).unwrap();
            // The goal positions of j and k are on the same lines
            // -- and j is on the left of k
            let goal_j_line = goal_j / size;
            if goal_j_line == line
                && goal_j_line == (goal_k / size)
                && goal_j > 0
                && goal_j - 1 == goal_k
            {
                linear_conflicts += 1.;
            }
        }
        // --  and row for vertical conflicts
        if (index + size) < total_len {
            // j is on top, k is down
            let k = node[index + size];
            // Ignore empty cell
            if j == 0 || k == 0 {
                continue;
            }
            let column = index % size;
            let goal_j = goal.iter().position(|&v| j == v).unwrap();
            let goal_k = goal.iter().position(|&v| k == v).unwrap();
            // The goal positions of j and k are on the same lines
            // -- and j is on the left of k
            let goal_j_line = goal_j % size;
            if goal_j_line == column
                && goal_j_line == (goal_k % size)
                && goal_j >= size
                && goal_j - size == goal_k
            {
                linear_conflicts += 1.;
            }
        }
    }

    manhattan_distance + (linear_conflicts * 2.)
}

#[test]
fn linear_conflict_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(linear_conflict(3, &left, &right), 1.)
}

#[test]
fn linear_conflict_two() {
    let left = vec![18, 8, 7, 4, 9, 6, 12, 24, 11];
    let right = vec![12, 24, 18, 8, 7, 4, 11, 9, 6];
    assert_eq!(linear_conflict(3, &left, &right), 16.)
}

#[test]
fn linear_conflict_three() {
    let left = vec![2, 1, 3, 8, 0, 4, 7, 6, 5];
    let right = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
    assert_eq!(linear_conflict(3, &left, &right), 4.)
}

#[test]
fn linear_conflict_four() {
    let left = vec![8, 2, 3, 1, 0, 4, 7, 6, 5];
    let right = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
    assert_eq!(linear_conflict(3, &left, &right), 4.)
}
