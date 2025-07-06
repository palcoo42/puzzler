use std::{error::Error, fs::File, io::BufRead, io::BufReader, path::PathBuf};

// Puzzle which can be solved by Solver
pub trait Puzzle {
    fn name(&self) -> &str;

    // By default puzzle is not using an input file
    fn get_input_file_path(&self) -> Option<PathBuf> {
        None
    }

    // Parse the file content for the puzzle. It is typically used in solve_partX() methods
    fn parse_content(&mut self, _lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok("Not solved".into())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok("Not solved".into())
    }

    // Solve third part of the puzzle
    fn solve_part3(&mut self) -> Result<String, Box<dyn Error>> {
        Ok("Not solved".into())
    }

    // Parse input file
    fn parse_input_file(&mut self) -> Result<(), Box<dyn Error>> {
        // Read file
        if let Some(input_file_path) = self.get_input_file_path() {
            let file = File::open(&input_file_path).map_err(|e| {
                format!(
                    "Failed to open file '{}' [{}]",
                    input_file_path.display(),
                    e
                )
            })?;
            let reader = BufReader::new(file);

            let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();
            let lines = lines.map_err(|e| format!("Failed to read lines from file [{e}]"))?;

            // Parse content
            self.parse_content(lines)?;
        }

        Ok(())
    }
}
