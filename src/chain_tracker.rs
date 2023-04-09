use std::collections::BTreeMap;
use crate::search_node::Edge;

#[derive(Clone)]
pub struct ChainTracker {
    start_to_end: BTreeMap<usize, usize>,
    end_to_start: BTreeMap<usize, usize>,
}

impl ChainTracker {
    pub fn new(n: usize) -> Self {
        let start_to_end: BTreeMap<usize, usize> = 
            (0..n)
            .map(|i| (i, i))
            .collect();
        let end_to_start = start_to_end.clone();
        
        Self { start_to_end, end_to_start }
    }
    
    pub fn insert_edge(&mut self, new_edge: Edge) -> Edge {
        let (source, target) = new_edge;

        let chain_start = self.end_to_start[&source];
        let chain_end = self.start_to_end[&target];

        self.delete_index(source);
        self.delete_index(target); 
        
        self.start_to_end.insert(chain_start, chain_end);
        self.end_to_start.insert(chain_end, chain_start);

        let edge_to_forbid = (chain_end, chain_start);
        edge_to_forbid
    }
    
    fn delete_index(&mut self, index: usize) {
        self.start_to_end.remove(&index);
        self.end_to_start.remove(&index);
    }
}