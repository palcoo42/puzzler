use std::{error::Error, path::PathBuf};

use puzzler::{
    env::project,
    parsers::parser::Parser,
    puzzler::{puzzle::Puzzle, solver::Solver},
};

struct DemoPuzzle {
    numbers: Vec<i64>,
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

    fn get_input_file_path(&self) -> Option<PathBuf> {
        let path = project::get_project_file("examples/input.txt")
            .unwrap_or_else(|err| panic!("Failed to find project file [{}]", err));

        Some(path)
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.numbers = Parser::parse_lines_to_integer(lines)?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.numbers.iter().sum::<i64>().to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut solver = Solver::new(Box::new(DemoPuzzle::new()), 2);
    solver.run()
}
