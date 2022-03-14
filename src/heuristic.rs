use npuzzle::Node;

pub fn manhattan(node: &Node, goal: &Node) -> i32 {
    let mut sum = 0;
    for i in node.iter().zip(goal) {
        sum += (i.0 - i.1).abs();
    }
    sum
}

#[test]
fn manhattan_one() {
    let left = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
    let right = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
    assert_eq!(manhattan(&left, &right), 16)
}

#[test]
fn manhattan_two() {
    let left = vec![10, 20, 15, 10, 5];
    let right = vec![12, 24, 18, 8, 7];
    assert_eq!(manhattan(&left, &right), 13)
}
