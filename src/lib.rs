use solver::Solver;
use std::env;

mod file_utils;
mod search_node;
mod solver;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let mut solver = Solver::new(&args[1]);
    solver.solve();
}