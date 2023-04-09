use std::collections::BTreeSet;
use crate::chain_tracker::ChainTracker;

pub type Edge = (usize, usize);

#[derive(Clone)]
pub struct SearchNode {
    pub n: usize,
    pub cost: f64,
    pub adjacency_matrix: Vec<Vec<f64>>,
    pub edges_included: Vec<Edge>,
    pub zeros: BTreeSet<Edge>,
    pub chains: ChainTracker,
}

impl SearchNode {
    pub fn new(adjacency_matrix: Vec<Vec<f64>>) -> Self {
        let n = adjacency_matrix.len();

        let mut root = Self {
            n,
            cost: 0.,
            adjacency_matrix,
            edges_included: Vec::new(),
            zeros: BTreeSet::new(),
            chains: ChainTracker::new(n),
        };

        root.reduce_matrix();
        root
    }
    
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges_included.push(edge);
        let edge_to_forbid = self.chains.insert_edge(edge);
        if self.edges_included.len() < self.n-1 {
            self.forbid_edge(edge_to_forbid);
        }
        self.forbid_row_and_col(edge);
    }
    
    pub fn find_edge_to_include(&self) -> Option<Edge> {
        let mut max_cost = self.cost;
        let mut max_edge = None;

        for edge in self.zeros.iter() {
            let cost = self.cost_if_not_included(*edge);
            if cost >= max_cost {
                max_cost = cost;
                max_edge = Some(*edge);
            }
        }
        
        max_edge
    }
    
    fn cost_if_not_included(&self, edge: Edge) -> f64 {
        let mut not_included = self.clone();
        not_included.forbid_edge(edge);
        not_included.reduce_matrix();
        not_included.cost
    }
}