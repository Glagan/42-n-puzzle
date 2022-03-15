use crate::puzzle::Puzzle;
use npuzzle::{neighbors, Node};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

#[derive(Clone)]
struct NodeWithCost<'a> {
    depth: i32,
    cost: f64,
    node: Node,
    parent: Option<Box<&'a NodeWithCost<'a>>>,
}

impl NodeWithCost<'_> {
    pub fn start(puzzle: &Puzzle) -> NodeWithCost {
        NodeWithCost {
            depth: 0,
            node: puzzle.map.clone(),
            cost: puzzle.heuristic(&puzzle.map),
            parent: None,
        }
    }
}

impl Eq for NodeWithCost<'_> {}

impl PartialEq for NodeWithCost<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for NodeWithCost<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for NodeWithCost<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution {
    pub total_used_states: i32,
    pub biggest_state: i32,
    pub steps: Vec<Node>,
}

fn construct_solution(node: &NodeWithCost) -> Vec<Node> {
    let mut full_path = vec![node.node.clone()];

    let mut current = node;
    while let Some(parent) = &current.parent {
        current = parent;
        full_path.push(current.node.clone());
    }

    full_path.reverse();
    full_path
}

pub fn solve(puzzle: &Puzzle) -> Result<Solution, String> {
    let now = Instant::now();

    // Summary
    let mut total_used_states = 0;
    let mut biggest_state: i32 = 0;

    // State
    let mut graph: Vec<NodeWithCost> = vec![NodeWithCost::start(puzzle)];
    let mut open_set = BinaryHeap::new();
    open_set.push(graph.get(0).unwrap());

    // Iterate on each cells
    while let Some(current) = open_set.pop() {
        total_used_states += 1;
        if open_set.len() > biggest_state.try_into().unwrap() {
            biggest_state = open_set.len().try_into().unwrap();
        }

        // Check if it's the goal
        if current.node == puzzle.goal {
            return Ok(Solution {
                total_used_states,
                biggest_state,
                steps: construct_solution(&current),
            });
        }

        for neighbor in neighbors(puzzle.size, &current.node).into_iter().flatten() {
            let depth = current.depth + 1;
            graph.push(NodeWithCost {
                depth,
                cost: depth as f64 + puzzle.heuristic(&neighbor),
                node: neighbor,
                parent: Some(Box::new(current)),
            });
            // open_set.push(&neighbor_node);
        }

        // println!("it {}", it);
        if total_used_states % 1000 == 0 {
            // println!("{:#?}", best_cost_to_node);
            println!(
                "# Total number of states ever selected: {} in {:.2?}",
                total_used_states,
                now.elapsed()
            );
        }
    }

    Err(String::from("Failed to find a solution for this puzzle"))
}
