use std::error::Error;

use puzzler::puzzler::{puzzle::Puzzle, solver::Solver};

struct DemoPuzzle {
    numbers: Vec<i32>,
}

impl DemoPuzzle {
    pub fn new() -> Self {
        Self { numbers: vec![] }
    }
}

impl Puzzle for DemoPuzzle {
    fn name(&self) -> &str {
        "DemoPuzzle"
    }

    fn get_input_file_path(&self) -> Option<String> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Some(format!("{}/examples/input.txt", manifest_dir))
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.numbers = lines
            .iter()
            .map(|line| {
                line.parse::<i32>()
                    .unwrap_or_else(|_| panic!("Invalid i32 [{}]", line))
            })
            .collect();
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.numbers.iter().sum::<i32>().to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut solver = Solver::new(Box::new(DemoPuzzle::new()), 2);
    solver.run()
}
