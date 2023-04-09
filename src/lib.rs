use solver::Solver;

mod file_utils;
mod chain_tracker;
mod matrix_utils;
mod search_node;
mod solver;

pub fn run() {
    let mut solver = Solver::new("test1.txt");
    solver.solve();
}