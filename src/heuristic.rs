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

// Sum of the manhattan distance for each cell in the Node
// sum(abs(x - y))
pub fn manhattan(size: i32, node: &Node, goal: &Node) -> f64 {
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
                (x_n - x_g).abs() as f64 + (y_n - y_g).abs() as f64
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
