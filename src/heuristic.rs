use npuzzle::Node;

#[allow(dead_code)]
pub fn manhattan(node: &Node, goal: &Node) -> f64 {
    let mut sum = 0.;
    for i in node.iter().zip(goal) {
        if i.1 > &0 {
            sum += (i.0 - i.1).abs() as f64;
        }
    }
    sum
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

#[allow(dead_code)]
pub fn euclidean_distance(node: &Node, goal: &Node) -> f64 {
    let mut sum = 0.;
    for i in node.iter().zip(goal) {
        if i.1 > &0 {
            sum += ((i.0 - i.1) as f64).powf(2.);
        }
    }
    sum.sqrt()
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
