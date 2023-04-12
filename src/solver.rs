use crate::{search_node::{SearchNode, edge::{Edge, Reverse}}, file_utils::read_file};

pub struct Solver {
    best_cost: i32,
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
            best_cost: i32::MAX,
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
        if self.nodes_visited % 1_000_000 == 0 {
            println!("{} nodes visited, {} is best so far", self.nodes_visited, self.best_cost);
        }

        if current_node.edges_included.len() == current_node.n {
            self.update_best(current_node);
            return;
        }
        
        let next_edge = current_node.find_edge_to_include();
        if let Some((edge, cost_not_included)) = next_edge {
            if cost_not_included >= self.best_cost {
                self.include_and_recurse(current_node, edge);
            }
            else {
                self.include_and_recurse(current_node.clone(), edge);
                self.exclude_and_recurse(current_node, edge);
            }
        }
    }
    
    fn include_and_recurse(&mut self, mut included: SearchNode, edge: Edge) {
        included.add_edge(edge);
        if included.cost < self.best_cost {
            self.recurse(included);
        }
    }
    
    fn exclude_and_recurse(&mut self, mut excluded: SearchNode, edge: Edge) {
        excluded.reduce_by_edge(edge);
        if excluded.edges_included.is_empty() {
            excluded.reduce_by_edge(edge.reverse());
        }
        if excluded.cost < self.best_cost {
            self.recurse(excluded);
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