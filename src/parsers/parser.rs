use regex::Regex;
use std::error::Error;

use crate::grids::grid::Grid;

pub struct Parser {}

impl Parser {
    // Split lines by given line pattern
    pub fn split_lines_by(lines: Vec<String>, pattern: &str) -> Vec<String> {
        lines.into_iter().filter(|line| !line.eq(pattern)).collect()
    }

    // Parse every line to single integer
    pub fn parse_lines_to_integer(lines: Vec<String>) -> Result<Vec<isize>, Box<dyn Error>> {
        lines
            .into_iter()
            .map(|line| {
                line.parse::<isize>()
                    .map_err(|err| format!("Failed to parse '{line}' to isize [{err}]").into())
            })
            .collect()
    }

    // Parse every line to list of integers separated with pattern
    pub fn parse_lines_to_integers(
        lines: Vec<String>,
        pattern: &str,
    ) -> Result<Vec<Vec<isize>>, Box<dyn Error>> {
        lines
            .into_iter()
            .map(|line| {
                line.split(pattern)
                    .map(str::trim)
                    .map(|s| {
                        s.parse::<isize>()
                            .map_err(|err| format!("Failed to parse '{s}' to isize [{err}]").into())
                    })
                    .collect::<Result<Vec<isize>, Box<dyn Error>>>()
            })
            .collect()
    }

    // Parse every line to list of strings separated with pattern
    pub fn parse_lines_to_strings(
        lines: Vec<String>,
        pattern: &str,
    ) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let result = lines
            .into_iter()
            .map(|line| {
                line.split(pattern)
                    .map(str::trim)
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        Ok(result)
    }

    // Parse lines with using the provided regex. For evevey found match the 'parsing' function F
    // will be called. The result of the parsing function (custom type) will be used as a return
    // value from line parsing.
    pub fn parse_lines_with_regex<F, U>(
        lines: Vec<String>,
        regex: &str,
        func: F,
    ) -> Result<Vec<U>, Box<dyn Error>>
    where
        F: Fn(Vec<String>) -> Result<U, Box<dyn Error>>, // User decoding function of parameters found in regex
    {
        // Instantiate regex, once for all parsed lines
        let re = Regex::new(regex)?;

        let mut decoded = vec![];

        // Go through all lines
        for line in &lines {
            // Apply regex pattern, in case of no match report an error
            let captures = re.captures(line).ok_or_else(|| -> Box<dyn Error> {
                format!("Failed to parse line '{line}'").into()
            })?;

            let groups = captures
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, m)| {
                    m.map(|mat| mat.as_str().to_string())
                        .ok_or_else(|| format!("Missing capture group at index {i}").into())
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

            // Decode groups with user specific function. If decode fails, report an error.
            let values = func(groups)?;
            decoded.push(values);
        }

        Ok(decoded)
    }

    // Parse lines to Grid instance
    pub fn parse_lines_to_grid(lines: Vec<String>) -> Result<Grid, Box<dyn Error>> {
        let grid = lines
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Grid::new(grid)
    }
}

#[cfg(test)]
mod tests {

    use crate::grids::point::Point;

    use super::*;

    #[test]
    fn test_split_lines_by_empty_line() {
        const DELIMITER: &str = "";

        let lines = vec![
            "1".to_string(),
            DELIMITER.to_string(),
            "2".to_string(),
            DELIMITER.to_string(),
            "3".into(),
        ];

        let split = Parser::split_lines_by(lines, DELIMITER);

        assert_eq!(
            split,
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        )
    }

    #[test]
    fn test_parse_lines_with_regex() {
        let lines = vec![
            "Button A: X+77, Y+52".to_string(),
            "Button A: X+98, Y+61".to_string(),
            "Button A: X+54, Y+20".to_string(),
        ];

        #[derive(PartialEq, Debug)]
        struct Button {
            x: isize,
            y: isize,
        }

        let regex = r#"^Button A: X\+(\d+), Y\+(\d+)"#;

        let result = Parser::parse_lines_with_regex(lines, regex, |params| {
            if params.len() != 2 {
                return Err(format!(
                    "Invalid number of parameters '{}', expected: 2",
                    params.len()
                )
                .into());
            }

            Ok(Button {
                x: params[0].parse::<isize>()?,
                y: params[1].parse::<isize>()?,
            })
        });

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Button { x: 77, y: 52 },
                Button { x: 98, y: 61 },
                Button { x: 54, y: 20 }
            ]
        );
    }

    #[test]
    fn test_parse_lines_with_regex_error() {
        let lines = vec![
            "Button A: X+77, Y+52".to_string(),
            "Button A: X+98, Y+61".to_string(),
            "Button A: X+54, Y+20".to_string(),
        ];

        let regex = r#"^Button A: X\+(\d+), Y\+(\d+)"#;

        let result: Result<Vec<isize>, Box<dyn Error>> =
            Parser::parse_lines_with_regex(lines, regex, |_| Err("Error".into()));

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lines_to_integer() {
        let lines = vec!["-1".to_string(), "2".to_string(), "3".to_string()];

        let result = Parser::parse_lines_to_integer(lines);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![-1, 2, 3]);
    }

    #[test]
    fn test_parse_lines_to_integer_error() {
        let lines = vec!["-1".to_string(), "2oops".to_string(), "3".to_string()];

        let result = Parser::parse_lines_to_integer(lines);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lines_to_integers() {
        let lines = vec!["-1 2 3 -4 42".to_string(), "2".to_string()];

        let result = Parser::parse_lines_to_integers(lines, " ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![vec![-1, 2, 3, -4, 42], vec![2]]);
    }

    #[test]
    fn test_parse_lines_to_integers_error() {
        let lines = vec![
            "-1 2 3 -4 42".to_string(),
            "2".to_string(),
            "3 oops".to_string(),
        ];

        let result = Parser::parse_lines_to_integers(lines, " ");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lines_to_strings() {
        let lines = vec!["-1 2 3 -4 42".to_string(), "2".to_string()];

        let result = Parser::parse_lines_to_strings(lines, " ");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                vec![
                    "-1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "-4".to_string(),
                    "42".to_string()
                ],
                vec!["2".to_string()]
            ]
        );
    }

    #[test]
    fn test_parse_lines_to_grid() {
        let lines = vec![
            "..#.S#".to_string(),
            "......".to_string(),
            "E#...#".to_string(),
        ];

        let result = Parser::parse_lines_to_grid(lines);
        assert!(result.is_ok());

        let grid = result.unwrap();
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 6);
        assert_eq!(grid[Point { x: 0, y: 0 }], '.');
        assert_eq!(grid[Point { x: 4, y: 0 }], 'S');
        assert_eq!(grid[Point { x: 5, y: 0 }], '#');
        assert_eq!(grid[Point { x: 1, y: 1 }], '.');
        assert_eq!(grid[Point { x: 0, y: 2 }], 'E');
        assert_eq!(grid[Point { x: 1, y: 2 }], '#');
        assert_eq!(grid[Point { x: 5, y: 2 }], '#');
    }
}
