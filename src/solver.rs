use crate::{search_node::{Edge, SearchNode, reverse_edge}, file_utils::read_file};

pub struct Solver {
    best_cost: f64,
    best_edges: Vec<Edge>,
    nodes_visited: usize,
    root: SearchNode,
    equals_best: usize,
}

impl Solver {
    pub fn new(file_name: &str) -> Self {
        let initial_matrix = read_file(file_name);
        let n = initial_matrix.len();
        println!("Successfully read matrix of dimension {}", n);
        
        let root = SearchNode::new(initial_matrix);
        println!("Initial lower bound: {}", root.cost);
        
        Self {
            best_cost: f64::INFINITY,
            best_edges: Vec::new(),
            nodes_visited: 0,
            root,
            equals_best: 0,
        }
    }

    pub fn solve(&mut self) {
        self.recurse(self.root.clone());
        println!("Minimum possible distance is {}", self.best_cost);
        println!("Included edges are {:?}", self.best_edges);
        println!("Visited {} nodes", self.nodes_visited);
        println!("{} equaled best", self.equals_best);
    }
    
    fn recurse(&mut self, mut current_node: SearchNode) {
        self.nodes_visited += 1;

        if current_node.edges_included.len() == current_node.n {
            self.update_best(current_node);
            return;
        }
        
        let next_edge = current_node.find_edge_to_include();
        if let Some(edge) = next_edge {
            let mut included = current_node.clone();

            included.add_edge(edge);
            included.reduce_matrix();

            if included.cost < self.best_cost {
                self.recurse(included.clone());
            }
            
            let mut not_included = current_node;
            if not_included.edges_included.is_empty() {
                not_included.reduce_by_edge(reverse_edge(edge));
            }
            not_included.reduce_by_edge(edge);

            if not_included.cost < self.best_cost {
                self.recurse(not_included);
            }
        }
    }

    fn update_best(&mut self, current_node: SearchNode) {
        if current_node.cost == self.best_cost {
            self.equals_best += 1;
        }
        if current_node.cost < self.best_cost {
            self.best_cost = current_node.cost;
            self.best_edges = current_node.edges_included
        }
    }
}