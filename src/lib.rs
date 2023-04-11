use solver::Solver;

mod file_utils;
mod chain_tracker;
mod matrix_utils;
mod search_node;
mod solver;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn run() {
    let mut solver = Solver::new("test.txt");
    solver.solve();
}