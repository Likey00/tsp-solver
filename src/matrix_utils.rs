use crate::search_node::{SearchNode, Edge};

impl SearchNode {
    pub fn reduce_matrix(&mut self) {
        self.reduce_rows();
        self.reduce_cols();
    }
    
    pub fn forbid_edge(&mut self, edge: Edge) {
        let (source, target) = edge;
        self.adjacency_matrix[source][target] = f64::INFINITY;
        self.zeros.remove(&edge);
    }
    
    pub fn forbid_row_and_col(&mut self, edge: Edge) {
        let (source, target) = edge;
        for i in 0..self.n {
            self.adjacency_matrix[source][i] = f64::INFINITY;
            self.adjacency_matrix[i][target] = f64::INFINITY;

            self.zeros.remove(&(source, i));
            self.zeros.remove(&(i, target));
        }
    }

    fn reduce_rows(&mut self) {
        for i in 0..self.n {
            let min_in_row = self.get_row_min(i);
            if min_in_row == f64::INFINITY { continue; }
            self.update_row_and_zeros(i, min_in_row);
            self.cost += min_in_row;
        }
    }
    
    fn reduce_cols(&mut self) {
        for i in 0..self.n {
            let min_in_col = self.get_col_min(i);
            if min_in_col == f64::INFINITY { continue; }
            self.update_col_and_zeros(i, min_in_col);
            self.cost += min_in_col;
        }
    }

    fn update_row_and_zeros(&mut self, row: usize, amt: f64) {
        for i in 0..self.n {
            self.adjacency_matrix[row][i] -= amt;
            if self.adjacency_matrix[row][i] == 0. {
                self.zeros.insert((row, i));
            }
        }
    }

    fn update_col_and_zeros(&mut self, col: usize, amt: f64) {
        for i in 0..self.n {
            self.adjacency_matrix[i][col] -= amt;
            if self.adjacency_matrix[i][col] == 0. {
                self.zeros.insert((i, col));
            }
        }
    }
    
    fn get_row_min(&self, row: usize) -> f64 {
        (0..self.n)
            .map(|i| self.adjacency_matrix[row][i])
            .min_by(|a, b| a.total_cmp(b))
            .unwrap()
    }
    
    fn get_col_min(&self, col: usize) -> f64 {
        (0..self.n)
            .map(|i| self.adjacency_matrix[i][col])
            .min_by(|a, b| a.total_cmp(b))
            .unwrap()
    }
}