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
    parent: Option<&'a NodeWithCost<'a>>,
    _neighbors: Option<Vec<NodeWithCost<'a>>>,
}

impl<'a> NodeWithCost<'a> {
    pub fn start(puzzle: &Puzzle) -> NodeWithCost {
        NodeWithCost {
            depth: 0,
            node: puzzle.map.clone(),
            cost: puzzle.heuristic(&puzzle.map),
            parent: None,
            _neighbors: None,
        }
    }

    pub fn neighbors(&'a mut self, puzzle: &Puzzle) -> Vec<NodeWithCost<'a>> {
        if self._neighbors.is_none() {
            self._neighbors = Some(
                neighbors(puzzle.size, &self.node)
                    .into_iter()
                    .flatten()
                    .map(|neighbor| NodeWithCost {
                        depth: self.depth + 1,
                        cost: puzzle.heuristic(&neighbor),
                        node: neighbor,
                        parent: Some(self),
                        _neighbors: None,
                    })
                    .collect(),
            );
        }
        self._neighbors.unwrap()
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

    // Keep reference to the whole graph and only borrow in other structs
    let mut graph = NodeWithCost::start(puzzle);
    // State
    let mut open_set: BinaryHeap<*mut NodeWithCost> = BinaryHeap::new();
    open_set.push(&mut graph);

    // Iterate on each cells
    while let Some(current) = open_set.pop() {
        total_used_states += 1;
        if open_set.len() > biggest_state.try_into().unwrap() {
            biggest_state = open_set.len().try_into().unwrap();
        }

        // Check if it's the goal
        if (*current).node == puzzle.goal {
            return Ok(Solution {
                total_used_states,
                biggest_state,
                steps: construct_solution(&current),
            });
        }

        // If the current node neighbors are not generated, they were never explored
        if current._neighbors.is_none() {
            println!("# exploring new neighbors");
            // Just add all neighbors to the open_set and it will use them if they have a good cost
            for neighbor in &mut current.neighbors(puzzle) {
                open_set.push(Box::new(neighbor));
            }
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
