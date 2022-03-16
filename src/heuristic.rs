use npuzzle::Node;

pub fn manhattan(node: &Node, goal: &Node) -> f64 {
    node.iter()
        .zip(goal)
        .map(|(x, y)| (x - y).abs() as f64)
        .sum::<f64>()
}

#[test]
fn manhattan_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(manhattan(&left, &right), 16.)
}

#[test]
fn manhattan_two() {
    let left = vec![10, 20, 15, 10, 5];
    let right = vec![12, 24, 18, 8, 7];
    assert_eq!(manhattan(&left, &right), 13.)
}

pub fn euclidean_distance(node: &Node, goal: &Node) -> f64 {
    node.iter()
        .zip(goal)
        .map(|(x, y)| ((x - y) as f64).powf(2.))
        .sum::<f64>()
        .sqrt()
}

#[test]
fn euclidean_distance_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(euclidean_distance(&left, &right), 11.313708498984761)
}

#[test]
fn euclidean_distance_two() {
    let left = vec![10, 20, 15, 10, 5];
    let right = vec![12, 24, 18, 8, 7];
    assert_eq!(euclidean_distance(&left, &right), 6.082762530298219)
}

pub fn hamming(node: &Node, goal: &Node) -> f64 {
    node.iter()
        .zip(goal)
        .map(|(x, y)| if x != y { 1. } else { 0. })
        .sum()
}

#[test]
fn hamming_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(hamming(&left, &right), 2.)
}

#[test]
fn hamming_two() {
    let left = vec![10, 20, 15, 10, 5];
    let right = vec![12, 24, 18, 8, 7];
    assert_eq!(hamming(&left, &right), 5.)
}
