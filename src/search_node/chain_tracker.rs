use crate::search_node::Edge;

#[derive(Clone)]
pub struct ChainTracker {
    start_to_end: Vec<usize>,
    end_to_start: Vec<usize>,
}

impl ChainTracker {
    pub fn new(n: usize) -> Self {
        Self {
            start_to_end: (0..n).collect(),
            end_to_start: (0..n).collect(),
        }
    }
    
    pub fn insert_edge(&mut self, new_edge: Edge) -> Edge {
        let chain_start = self.end_to_start[new_edge.0];
        let chain_end = self.start_to_end[new_edge.1];

        self.start_to_end[chain_start] = chain_end;
        self.end_to_start[chain_end] = chain_start;

        (chain_end, chain_start)
    }
}