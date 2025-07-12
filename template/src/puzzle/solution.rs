use std::path::PathBuf;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "template"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("input/day_XX.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file input/day_XX.txt [{err}]")),
        )
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "Not solved");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "Not solved");
    }
}
