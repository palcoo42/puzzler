use std::error::Error;

use puzzler::puzzler::solver::Solver;

use crate::puzzle::solution::Solution;

mod puzzle;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle = Box::new(Solution::new());
    let mut solver = Solver::new(puzzle, 2);
    solver.run()
}
