use crate::search_node::{SearchNode, Edge};

impl SearchNode {
    pub fn reduce_matrix(&mut self) {
        self.reduce_rows();
        self.reduce_cols();
    }
    
    pub fn reduce_by_edge(&mut self, edge: Edge) {
        self.forbid_edge(edge);
        let (source, target) = edge;
        
        if let Some(row_min) = self.get_row_min(source) {
            self.update_row_and_zeros(source, row_min);
            self.cost += row_min;
        }
        
        if let Some(col_min) = self.get_col_min(target) {
            self.update_col_and_zeros(target, col_min);
            self.cost += col_min;
        }
    }
    
    pub fn cost_after_deletion(&self, edge: Edge) -> i32 {
        let (source, target) = edge;
        
        let new_row_min = self.row_order.iter()
            .filter(|i| **i != source)
            .map(|i| self.adjacency_matrix[(*i, target)])
            .filter(|val| *val < i32::MAX / 2)
            .min();
        
        let new_col_min = self.col_order.iter()
            .filter(|i| **i != target)
            .map(|i| self.adjacency_matrix[(source, *i)])
            .filter(|val| *val < i32::MAX / 2)
            .min();

        self.cost + {
            new_row_min.unwrap_or(0)
            + new_col_min.unwrap_or(0)
        }
    }
    
    pub fn forbid_edge(&mut self, edge: Edge) {
        self.adjacency_matrix[edge] = i32::MAX;
        self.zeros.remove(&edge);
    }
    
    pub fn forbid_row_and_col(&mut self, edge: Edge) {
        let (source, target) = edge;
        for (i, row) in self.row_order.iter().enumerate() {
            if *row == source {
                self.row_order.swap_remove(i);
                break;
            }
        }
        for (i, col) in self.col_order.iter().enumerate() {
            if *col == target {
                self.col_order.swap_remove(i);
                break;
            }
        }

        self.zeros.retain(|(s, t)| *s != source && *t != target);
    }

    fn reduce_rows(&mut self) {
        for i in self.row_order.iter() {
            if let Some(min_in_row) = self.get_row_min(*i) {
                for j in self.col_order.iter() {
                    self.adjacency_matrix[(*i, *j)] -= min_in_row;
                    if self.adjacency_matrix[(*i, *j)] == 0 {
                        self.zeros.insert((*i, *j));
                    }
                }
                self.cost += min_in_row;
            }
        }
    }
    
    fn reduce_cols(&mut self) {
        for j in self.col_order.iter() {
            if let Some(min_in_col) = self.get_col_min(*j) {
                for i in self.row_order.iter() {
                    self.adjacency_matrix[(*i, *j)] -= min_in_col;
                    if self.adjacency_matrix[(*i, *j)] == 0 {
                        self.zeros.insert((*i, *j));
                    }
                }
                self.cost += min_in_col;
            }
        }
    }

    fn update_row_and_zeros(&mut self, row: usize, amt: i32) {
        for i in self.col_order.iter() {
            self.adjacency_matrix[(row, *i)] -= amt;
            if self.adjacency_matrix[(row, *i)] == 0 {
                self.zeros.insert((row, *i));
            }
        }
    }

    fn update_col_and_zeros(&mut self, col: usize, amt: i32) {
        for i in self.row_order.iter() {
            self.adjacency_matrix[(*i, col)] -= amt;
            if self.adjacency_matrix[(*i, col)] == 0 {
                self.zeros.insert((*i, col));
            }
        }
    }
    
    fn get_row_min(&self, row: usize) -> Option<i32> {
        self.col_order.iter()
            .map(|i| self.adjacency_matrix[(row, *i)])
            .filter(|val| *val < i32::MAX/2)
            .min()
    }
    
    fn get_col_min(&self, col: usize) -> Option<i32> {
        self.row_order.iter()
            .map(|i| self.adjacency_matrix[(*i, col)])
            .filter(|val| *val < i32::MAX/2)
            .min()
    }
}