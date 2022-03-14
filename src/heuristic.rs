use npuzzle::Node;

pub fn manhattan(node: &Node, goal: &Node) -> i32 {
    let mut sum = 0;
    for i in node.iter().zip(goal) {
        sum += (i.0 - i.1).abs();
    }
    sum
}
