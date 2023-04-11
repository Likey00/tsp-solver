use std::collections::{BTreeSet, HashSet};
use array2d::Array2D;

use crate::chain_tracker::ChainTracker;

pub type Edge = (usize, usize);

pub fn reverse_edge(edge: Edge) -> Edge {
    let (source, target) = edge;
    (target, source)
}

#[derive(Clone)]
pub struct SearchNode {
    pub n: usize,
    pub cost: i32,
    pub adjacency_matrix: Array2D<i32>,
    pub edges_included: Vec<Edge>,
    pub zeros: BTreeSet<Edge>,
    pub row_order: Vec<usize>,
    pub col_order: Vec<usize>,
    pub chains: ChainTracker,
}

impl SearchNode {
    pub fn new(adjacency_matrix: Vec<Vec<i32>>) -> Self {
        let n = adjacency_matrix.len();

        let mut root = Self {
            n,
            cost: 0,
            adjacency_matrix: Array2D::from_rows(&adjacency_matrix).unwrap(),
            edges_included: Vec::with_capacity(n*n),
            zeros: BTreeSet::new(),
            row_order: (0..n).collect(),
            col_order: (0..n).collect(),
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
        self.reduce_matrix();
    }
    
    pub fn find_edge_to_include(&mut self) -> Option<(Edge, i32)> {
        let mut max_cost = 0;
        let mut max_edge = None;

        for edge in self.zeros.iter() {
            let cost = self.cost_after_deletion(*edge);
            if cost > max_cost {
                max_cost = cost;
                max_edge = Some(*edge);
            }
        }
        
        Some((max_edge?, max_cost))
    }
}