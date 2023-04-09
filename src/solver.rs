use crate::{search_node::{Edge, SearchNode}, file_utils::read_file};

pub struct Solver {
    best_cost: f64,
    best_edges: Vec<Edge>,
    nodes_visited: usize,
    root: SearchNode,
}

impl Solver {
    pub fn new(file_name: &str) -> Self {
        let adjacency_matrix = read_file(file_name);
        println!("Successfully read matrix of dimension {}", adjacency_matrix.len());
        
        let root = SearchNode::new(adjacency_matrix);
        println!("Initial lower bound: {}", root.cost);
        
        Self {
            best_cost: f64::INFINITY,
            best_edges: Vec::new(),
            nodes_visited: 0,
            root,
        }
    }

    pub fn solve(&mut self) {
        self.recurse(self.root.clone());
        println!("Minimum possible distance is {}", self.best_cost);
        println!("Included edges are {:?}", self.best_edges);
        println!("Visited {} nodes", self.nodes_visited);
    }
    
    fn recurse(&mut self, current_node: SearchNode) {
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
                self.recurse(included);
            }
            
            let mut not_included = current_node.clone();
            not_included.forbid_edge(edge);
            not_included.reduce_matrix();

            if not_included.cost < self.best_cost {
                self.recurse(not_included);
            }
        }
    }

    fn update_best(&mut self, current_node: SearchNode) {
        if current_node.cost < self.best_cost {
            self.best_cost = current_node.cost;
            self.best_edges = current_node.edges_included.clone();
        }
    }
}