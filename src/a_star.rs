use crate::puzzle::Puzzle;
use npuzzle::Node;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone)]
struct NodeWithCost {
    depth: i32,
    cost: f64,
    node: Node,
    parent: Option<Rc<RefCell<NodeWithCost>>>,
    explored: bool,
}

impl NodeWithCost {
    pub fn start(puzzle: &Puzzle) -> NodeWithCost {
        NodeWithCost {
            depth: 0,
            node: puzzle.map.clone(),
            cost: 0.,
            parent: None,
            explored: false,
        }
    }
}

impl Eq for NodeWithCost {}

impl PartialEq for NodeWithCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap().reverse()
    }
}

impl PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.partial_cmp(&other.cost).unwrap().reverse())
    }
}

pub struct Solution {
    pub total_used_states: i32,
    pub biggest_state: usize,
    pub steps: Vec<Node>,
}

fn construct_solution(node: Rc<RefCell<NodeWithCost>>) -> Vec<Node> {
    let mut full_path = vec![];

    let mut current = Some(node);
    while let Some(node) = current {
        let borrowed = node.borrow();
        full_path.push(borrowed.node.clone());
        current = borrowed.parent.as_ref().map(Rc::clone);
    }

    full_path.reverse();
    full_path
}

pub fn solve(
    puzzle: &Puzzle,
    mode: &str,
    heuristic: fn(&Node, &Node) -> f64,
) -> Result<Solution, String> {
    let now = Instant::now();
    let mode = match mode {
        "greedy" => 1,
        "uniform" => 2,
        _ => 0,
    };

    // Summary
    let mut total_used_states = 0;
    let mut biggest_state: usize = 1; // 1 is the initial state

    // Keep reference to the whole graph and only borrow in other structs
    let graph = NodeWithCost::start(puzzle);
    // let mut nodes: Vec<Rc<NodeWithCost>> = Vec::new();
    // State
    let mut open_set: BinaryHeap<Rc<RefCell<NodeWithCost>>> = BinaryHeap::new();
    open_set.push(Rc::new(RefCell::new(graph)));

    // Iterate on each cells
    while let Some(current) = open_set.pop() {
        total_used_states += 1;

        // Check if it's the goal
        if current.borrow().node == puzzle.goal {
            return Ok(Solution {
                total_used_states,
                biggest_state,
                steps: construct_solution(current),
            });
        }

        // If the current node neighbors are not generated, they were never explored
        if !current.borrow().explored {
            current.borrow_mut().explored = true;
            let neighbors: Vec<Rc<RefCell<NodeWithCost>>> = puzzle
                .neighbors(&current.borrow().node)
                .into_iter()
                .flatten()
                .map(|neighbor| {
                    let depth = current.borrow().depth + 1;
                    Rc::new(RefCell::new(NodeWithCost {
                        depth,
                        cost: match mode {
                            0 => depth as f64 + heuristic(&neighbor, &puzzle.goal),
                            1 => heuristic(&neighbor, &puzzle.goal), // Ignore depth
                            _ => depth as f64,                       // Ignore heuristic
                        },
                        node: neighbor,
                        parent: Some(Rc::clone(&current)),
                        explored: false,
                    }))
                })
                .collect();
            // Just add all neighbors to the open_set and it will use them if they have a good cost in the priority queue
            for neighbor in neighbors {
                open_set.push(Rc::clone(&neighbor))
            }
            if open_set.len() > biggest_state {
                biggest_state = open_set.len() + 1;
            }
        }

        // println!("it {}", it);
        if total_used_states % 10000 == 0 {
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
