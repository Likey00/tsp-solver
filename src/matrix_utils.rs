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
    
    pub fn cost_after_deletion(&mut self, edge: Edge) -> f64 {
        let (source, target) = edge;
        
        let temp = self.adjacency_matrix[source][target];
        self.adjacency_matrix[source][target] = f64::INFINITY;
        let new_cost = self.cost + {
            self.get_row_min(source).unwrap_or(0.)
            + self.get_col_min(target).unwrap_or(0.)
        };
        self.adjacency_matrix[source][target] = temp;

        new_cost
    }
    
    pub fn forbid_edge(&mut self, edge: Edge) {
        let (source, target) = edge;
        self.adjacency_matrix[source][target] = f64::INFINITY;
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
        
        self.zeros = self.zeros
            .iter()
            .filter(|(s, t)| *s != source && *t != target)
            .map(|(s, t)| (*s, *t))
            .collect();
    }

    fn reduce_rows(&mut self) {
        for i in self.row_order.clone() {
            if let Some(min_in_row) = self.get_row_min(i) {
                self.update_row_and_zeros(i, min_in_row);
                self.cost += min_in_row;
            }
        }
    }
    
    fn reduce_cols(&mut self) {
        for i in self.col_order.clone() {
            if let Some(min_in_col) = self.get_col_min(i) {
                self.update_col_and_zeros(i, min_in_col);
                self.cost += min_in_col;
            }
        }
    }

    fn update_row_and_zeros(&mut self, row: usize, amt: f64) {
        for i in self.col_order.iter() {
            if self.adjacency_matrix[row][*i] == f64::INFINITY {
                continue;
            }
            self.adjacency_matrix[row][*i] -= amt;
            if self.adjacency_matrix[row][*i] == 0. {
                self.zeros.insert((row, *i));
            }
        }
    }

    fn update_col_and_zeros(&mut self, col: usize, amt: f64) {
        for i in self.row_order.iter() {
            if self.adjacency_matrix[*i][col] == f64::INFINITY {
                continue;
            }
            self.adjacency_matrix[*i][col] -= amt;
            if self.adjacency_matrix[*i][col] == 0. {
                self.zeros.insert((*i, col));
            }
        }
    }
    
    fn get_row_min(&self, row: usize) -> Option<f64> {
        self.col_order.iter()
            .map(|i| self.adjacency_matrix[row][*i])
            .filter(|val| *val != f64::INFINITY)
            .min_by(|a, b| a.total_cmp(b))
    }
    
    fn get_col_min(&self, col: usize) -> Option<f64> {
        self.row_order.iter()
            .map(|i| self.adjacency_matrix[*i][col])
            .filter(|val| *val != f64::INFINITY)
            .min_by(|a, b| a.total_cmp(b))
    }
}