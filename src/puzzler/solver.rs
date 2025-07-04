use std::path::PathBuf;
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

use crate::puzzler::puzzle::Puzzle;

// Maximum number of puzzle parts
const MAX_PUZZLE_PARTS: u32 = 3;

// Solver for challenges
pub struct Solver {
    parts: u32,
    puzzle: Box<dyn Puzzle>,
}

impl Solver {
    // Create new solver instance to solve puzzle
    pub fn new(puzzle: Box<dyn Puzzle>, parts: u32) -> Self {
        if !(1..=MAX_PUZZLE_PARTS).contains(&parts) {
            panic!(
                "Invalid number of puzzle parts '{}'. Allowed range is range <1,{}>",
                parts, MAX_PUZZLE_PARTS
            )
        }

        Self { parts, puzzle }
    }

    pub fn read_input_file(input_file_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(input_file_path).map_err(|e| {
            format!(
                "Failed to open file '{}' [{}]",
                input_file_path.display(),
                e
            )
        })?;
        let reader = BufReader::new(file);

        Ok(reader.lines().collect::<Result<_, _>>()?)
    }

    // Solve puzzle
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // Show puzzle name
        println!("{}", self.puzzle.name());
        println!("{}", "=".repeat(self.puzzle.name().len()));

        self.puzzle.parse_input_file()?;

        // Solve puzzle parts
        for part in 1..=self.parts {
            print!("Part {}: ", part);

            let result = match part {
                1 => self.puzzle.solve_part1()?,
                2 => self.puzzle.solve_part2()?,
                3 => self.puzzle.solve_part3()?,
                oops => panic!("Unexpected part '{}'", oops),
            };

            println!("{}", result);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Minimalistic test puzzle
    struct TestPuzzle {}
    impl Puzzle for TestPuzzle {
        fn name(&self) -> &str {
            "TestPuzzle"
        }
    }

    #[test]
    fn test_new_valid_parts() {
        let _solver = Solver::new(Box::new(TestPuzzle {}), 1);
        let _solver = Solver::new(Box::new(TestPuzzle {}), 2);
        let _solver = Solver::new(Box::new(TestPuzzle {}), 3);
    }

    #[test]
    #[should_panic]
    fn test_new_too_little_parts() {
        let _solver = Solver::new(Box::new(TestPuzzle {}), 0);
    }

    #[test]
    #[should_panic]
    fn test_new_too_many_parts() {
        let _solver = Solver::new(Box::new(TestPuzzle {}), 4);
    }
}
