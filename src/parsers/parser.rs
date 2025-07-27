use regex::Regex;
use std::error::Error;

use crate::grids::grid::Grid;

pub struct Parser {}

impl Parser {
    // Parse every line to single integer
    pub fn parse_lines_to_integer(lines: Vec<String>) -> Result<Vec<isize>, Box<dyn Error>> {
        let integers = Self::parse_lines_to_integers(lines)?;

        // Take only first integer from every line
        integers
            .iter()
            .map(|line| -> Result<isize, Box<dyn Error>> {
                if line.len() != 1 {
                    return Err(format!("Exactly one integer expected '{line:?}'").into());
                }

                Ok(line[0])
            })
            .collect::<Result<Vec<_>, _>>()
    }

    // Parse every line to list of integers
    pub fn parse_lines_to_integers(lines: Vec<String>) -> Result<Vec<Vec<isize>>, Box<dyn Error>> {
        let regex = Regex::new(r"[+-]?\d+")?;
        let mut numbers = Vec::new();

        for line in lines {
            // If line contains non-number report error
            if line.chars().any(|c| c.is_alphabetic()) {
                return Err(format!("Line '{line}' contains non-number character(s)").into());
            }

            // Parse numbers, there can be any number of them
            let line_numbers = regex
                .find_iter(line.as_str())
                .map(|s| -> Result<_, Box<dyn Error>> {
                    s.as_str().parse::<isize>().map_err(|err| {
                        format!("Failed to parse '{}' to isize [{err}]", s.as_str()).into()
                    })
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

            numbers.push(line_numbers);
        }

        Ok(numbers)
    }

    // Parse every line to single unsigned integer
    pub fn parse_lines_to_unsigned_integer(
        lines: Vec<String>,
    ) -> Result<Vec<usize>, Box<dyn Error>> {
        let integers = Self::parse_lines_to_unsigned_integers(lines)?;

        // Take only first integer from every line
        integers
            .iter()
            .map(|line| -> Result<usize, Box<dyn Error>> {
                if line.len() != 1 {
                    return Err(format!("Exactly one integer expected '{line:?}'").into());
                }

                Ok(line[0])
            })
            .collect::<Result<Vec<_>, _>>()
    }

    // Parse every line to list of unsigned integers
    pub fn parse_lines_to_unsigned_integers(
        lines: Vec<String>,
    ) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
        let regex = Regex::new(r"\d+")?;
        let mut numbers = Vec::new();

        for line in lines {
            // If line contains non-number report error
            if line.chars().any(|c| c.is_alphabetic()) {
                return Err(format!("Line '{line}' contains non-number character(s)").into());
            }

            // Parse numbers, there can be any number of them
            let line_numbers = regex
                .find_iter(line.as_str())
                .map(|s| -> Result<_, Box<dyn Error>> {
                    s.as_str().parse::<usize>().map_err(|err| {
                        format!("Failed to parse '{}' to usize [{err}]", s.as_str()).into()
                    })
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

            numbers.push(line_numbers);
        }

        Ok(numbers)
    }

    // Parse every line to list of strings separated with pattern
    pub fn parse_lines_to_strings(
        lines: Vec<String>,
        pattern: &str,
    ) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let result = lines
            .into_iter()
            .map(|line| {
                line.trim()
                    .split(pattern)
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

    pub fn parse_lines_to_grid_str(lines: &[&str]) -> Result<Grid, Box<dyn Error>> {
        let grid = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Grid::new(grid)
    }

    // Group lines by empty lines
    pub fn group_lines(lines: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = Vec::new();
        let mut group = Vec::new();

        for line in lines {
            // Pattern terminates given group
            match line.is_empty() {
                true => {
                    // End of group
                    if !group.is_empty() {
                        groups.push(group.clone());
                        group.clear();
                    }
                }
                false => {
                    // Still in the group
                    group.push(line);
                }
            }
        }

        // Add last group if available
        if !group.is_empty() {
            groups.push(group);
        }

        groups
    }

    pub fn decode_line_to_unsigned_integer(line: &str, pat: &str) -> Result<usize, Box<dyn Error>> {
        // Find position after the pattern
        let pos = line.find(pat).ok_or_else(|| -> Box<dyn Error> {
            format!("Pattern '{pat}' not found in '{line}'").into()
        })?;

        // From the end of pattern decode unsigned integer
        let number = line[pos + pat.len()..]
            .trim()
            .parse::<usize>()
            .map_err(|err| -> Box<dyn Error> {
                format!("Failed to parse '{line}' after pattern '{pat}', substring '{}' to usize [{err}]", &line[pos + pat.len()..]).into()
            })?;

        Ok(number)
    }

    pub fn decode_line_to_signed_integer(line: &str, pat: &str) -> Result<isize, Box<dyn Error>> {
        // Find position after the pattern
        let pos = line.find(pat).ok_or_else(|| -> Box<dyn Error> {
            format!("Pattern '{pat}' not found in '{line}'").into()
        })?;

        // From the end of pattern decode signed integer
        let number = line[pos + pat.len()..]
            .trim()
            .parse::<isize>()
            .map_err(|err| -> Box<dyn Error> {
                format!("Failed to parse '{line}' after pattern '{pat}', substring '{}' to isize [{err}]", &line[pos + pat.len()..]).into()
            })?;

        Ok(number)
    }

    pub fn decode_line_to_string(line: &str, pat: &str) -> Result<String, Box<dyn Error>> {
        // Find position after the pattern
        let pos = line.find(pat).ok_or_else(|| -> Box<dyn Error> {
            format!("Pattern '{pat}' not found in '{line}'").into()
        })?;

        // From the end of pattern decode String
        let text = line[pos + pat.len()..].trim();

        Ok(text.to_string())
    }
}

#[cfg(test)]
mod tests {

    use crate::grids::point::Point;

    use super::*;

    #[test]
    fn test_group_lines_empty_line() {
        let lines = vec![
            "1".to_string(),
            "2".to_string(),
            "3".into(),
            "".to_string(),
            "Something else".to_string(),
        ];

        let groups = Parser::group_lines(lines);

        assert_eq!(groups.len(), 2);
        assert_eq!(
            groups[0],
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
        assert_eq!(groups[1], vec!["Something else".to_string()])
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
        let lines = vec![
            "-1".to_string(),
            " 2".to_string(),
            " 3".to_string(),
            " 4 ".to_string(),
        ];

        let result = Parser::parse_lines_to_integer(lines);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![-1, 2, 3, 4]);
    }

    #[test]
    fn test_parse_lines_to_integer_error() {
        let lines = vec!["-1".to_string(), "2oops".to_string(), "3".to_string()];

        let result = Parser::parse_lines_to_integer(lines);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lines_to_integers() {
        let lines = vec![" -1 2 3 -4 42 ".to_string(), " 2".to_string()];

        let result = Parser::parse_lines_to_integers(lines);
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

        let result = Parser::parse_lines_to_integers(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lines_to_unsigned_integer() {
        let lines = vec![
            " 1 ".to_string(),
            "2 ".to_string(),
            " 3".to_string(),
            "4".to_string(),
        ];

        let result = Parser::parse_lines_to_integer(lines);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_parse_lines_to_unsigned_integer_error() {
        let lines = vec!["1".to_string(), "2oops".to_string(), "3".to_string()];

        let result = Parser::parse_lines_to_integer(lines);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lines_to_unsigned_integers() {
        let lines = vec![
            "1".to_string(),
            " 2".to_string(),
            "3 ".to_string(),
            " 4 ".to_string(),
            "   5  6      7    8  ".to_string(),
        ];

        let result = Parser::parse_lines_to_unsigned_integers(lines);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![vec![1], vec![2], vec![3], vec![4], vec![5, 6, 7, 8]]
        );
    }

    #[test]
    fn test_parse_lines_to_unsigned_integers_error() {
        let lines = vec![
            "1 2 3 4 42".to_string(),
            "2".to_string(),
            "3 oops".to_string(),
        ];

        let result = Parser::parse_lines_to_unsigned_integers(lines);
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

    #[test]
    fn test_parse_lines_to_grid_str() {
        let lines = vec!["..#.S#", "......", "E#...#"];

        let result = Parser::parse_lines_to_grid_str(&lines);
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

    #[test]
    fn test_decode_line_to_unsigned_integer() {
        let result = Parser::decode_line_to_unsigned_integer("Age: 42", "Age:");
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_decode_line_to_signed_integer() {
        let result = Parser::decode_line_to_signed_integer("Price: -100", "Price: ");
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap(), -100);
    }

    #[test]
    fn test_decode_line_to_string() {
        let result = Parser::decode_line_to_string("Name: N/A", "Name:");
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap(), String::from("N/A"));
    }
}
