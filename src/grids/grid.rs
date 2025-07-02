use std::{
    error::Error,
    ops::{Index, IndexMut},
};

use super::{direction::Direction, point::Point};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Grid {
    rows: usize,
    cols: usize,
    internal: Vec<Vec<char>>,
}

impl Grid {
    /// Create new grid, rows and columns count is deduced from input data
    pub fn new(data: Vec<Vec<char>>) -> Result<Self, Box<dyn Error>> {
        // Check input data
        if data.is_empty() {
            return Err("Grid is empty".into());
        }

        if data[0].is_empty() {
            return Err("Grid[0] is empty".into());
        }

        let rows = data.len();
        let cols = data[0].len();

        Ok(Self {
            rows,
            cols,
            internal: data,
        })
    }

    pub fn new_with<F>(rows: usize, cols: usize, func: F) -> Result<Self, Box<dyn Error>>
    where
        F: Fn(Point) -> char,
    {
        let internal = (0..rows)
            .map(|i| {
                {
                    (0..cols)
                        .map(|j| {
                            func(Point {
                                x: j as isize,
                                y: i as isize,
                            })
                        })
                        .collect::<Vec<_>>()
                }
            })
            .collect::<Vec<_>>();

        Ok(Self {
            rows,
            cols,
            internal,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn fill(&mut self, data: &[(Point, char)]) -> Result<(), Box<dyn Error>> {
        // Avoid changing of the grid if there is invalid point
        for (point, _) in data {
            if !self.is_point_in_grid(point) {
                return Err(format!("Point {:?} is not in the grid", point).into());
            }
        }

        for (point, value) in data {
            self[*point] = *value;
        }

        Ok(())
    }

    pub fn is_point_in_grid(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols as isize && point.y >= 0 && point.y < self.rows as isize
    }

    pub fn neighbor(&self, point: &Point, direction: &Direction) -> Option<(Point, Direction)> {
        self.neighbor_if(point, direction, |_, _| true)
    }

    pub fn neighbor_if<F>(
        &self,
        point: &Point,
        direction: &Direction,
        func: F,
    ) -> Option<(Point, Direction)>
    where
        F: Clone + Fn(&Point, &Direction) -> bool,
    {
        // Calculate coordinates of the neighbor
        let neighbor = point.neighbor(direction);

        // If neighbor is within grid return it
        match self.is_point_in_grid(&neighbor) && func(&neighbor, direction) {
            true => Some((neighbor, *direction)),
            false => None,
        }
    }

    pub fn neighbors(&self, point: &Point, directions: &[Direction]) -> Vec<(Point, Direction)> {
        directions
            .iter()
            .filter_map(|direction| self.neighbor(point, direction))
            .collect::<Vec<_>>()
    }

    pub fn neighbors_if<F>(
        &self,
        point: &Point,
        directions: &[Direction],
        func: F,
    ) -> Vec<(Point, Direction)>
    where
        F: Clone + Fn(&Point, &Direction) -> bool,
    {
        directions
            .iter()
            .filter_map(|direction| self.neighbor_if(point, direction, func.clone()))
            .collect::<Vec<_>>()
    }

    pub fn get_if<F>(&self, func: F) -> Vec<Point>
    where
        F: Copy + Fn(char) -> bool,
    {
        self.internal
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, c)| match func(*c) {
                        true => Some(Point {
                            x: j as isize,
                            y: i as isize,
                        }),
                        false => None,
                    })
            })
            .collect()
    }

    /// Get positions of all values from the grid
    pub fn get_value(&self, value: char) -> Vec<Point> {
        self.get_value_if(value, || true)
    }

    pub fn get_value_if<F>(&self, value: char, func: F) -> Vec<Point>
    where
        F: Copy + Fn() -> bool,
    {
        self.internal
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, c)| match *c == value && func() {
                        true => Some(Point {
                            x: j as isize,
                            y: i as isize,
                        }),
                        false => None,
                    })
            })
            .collect()
    }

    /// Print grid to the console.
    pub fn print(&self) {
        self.print_with_visited(&[]);
    }

    /// Print grid to the console. If point is in visited collection show 'O' for this point.
    pub fn print_with_visited(&self, visited: &[Point]) {
        // Go through all rows
        for i in 0..self.rows {
            // Format whole line and print it only once to the console to speedup writesS
            let line = self.internal[i]
                .iter()
                .enumerate()
                .map(|(j, c)| {
                    match visited.contains(&Point {
                        x: j as isize,
                        y: i as isize,
                    }) {
                        true => 'O',
                        false => *c,
                    }
                })
                .collect::<String>();

            println!("{}", line);
        }
    }
}

impl Index<Point> for Grid {
    type Output = char;

    fn index(&self, index: Point) -> &Self::Output {
        &self.internal[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.internal[index.y as usize][index.x as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::parser::Parser;

    use super::*;

    fn build_grid() -> Grid {
        Grid::new(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]).unwrap()
    }

    #[test]
    fn test_new_with() {
        let result = Grid::new_with(2, 3, |_| '.');

        assert!(result.is_ok());

        let grid = result.unwrap();
        assert_eq!(grid.rows, 2);
        assert_eq!(grid.cols, 3);

        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(
                    grid[Point { x: j, y: i }],
                    '.',
                    "Failed at (x: {}, y: {})",
                    j,
                    i
                );
            }
        }
    }

    #[test]
    fn test_index() {
        let mut grid = build_grid();

        assert_eq!(grid.rows, 2);
        assert_eq!(grid.cols, 3);
        assert_eq!(grid[Point::new(0, 0)], 'a');
        assert_eq!(grid[Point::new(2, 1)], 'f');

        grid[Point::new(0, 0)] = 'x';
        assert_eq!(grid[Point::new(0, 0)], 'x');
    }

    #[test]
    fn test_is_point_in_grid() {
        let grid = build_grid();

        assert!(grid.is_point_in_grid(&Point { x: 0, y: 0 }));
        assert!(grid.is_point_in_grid(&Point { x: 2, y: 0 }));
        assert!(grid.is_point_in_grid(&Point { x: 0, y: 1 }));
        assert!(grid.is_point_in_grid(&Point { x: 2, y: 1 }));
    }

    #[test]
    fn test_is_point_not_in_grid() {
        let grid = build_grid();

        assert!(!grid.is_point_in_grid(&Point { x: -1, y: 0 }));
        assert!(!grid.is_point_in_grid(&Point { x: 0, y: -1 }));
        assert!(!grid.is_point_in_grid(&Point { x: 3, y: 1 }));
        assert!(!grid.is_point_in_grid(&Point { x: 2, y: 2 }));
    }

    #[test]
    fn test_neighbor_if_true() {
        let grid = build_grid();

        let result = grid.neighbor_if(&Point::new(0, 0), &Direction::East, |_, _| true);
        assert!(result.is_some(), "result: {:?}", result);
        assert_eq!(result.unwrap(), (Point::new(1, 0), Direction::East));

        let result = grid.neighbor_if(&Point::new(0, 0), &Direction::West, |_, _| true);
        assert!(result.is_none(), "result: {:?}", result);
    }

    #[test]
    fn test_neighbor_if_false() {
        let grid = build_grid();

        let result = grid.neighbor_if(&Point::new(0, 0), &Direction::East, |_, _| false);
        assert!(result.is_none(), "result: {:?}", result);

        let result = grid.neighbor_if(&Point::new(0, 0), &Direction::West, |_, _| false);
        assert!(result.is_none(), "result: {:?}", result);
    }

    #[test]
    fn test_neighbor() {
        let grid = build_grid();

        let result = grid.neighbor(&Point::new(0, 0), &Direction::East);
        assert!(result.is_some(), "result: {:?}", result);
        assert_eq!(result.unwrap(), (Point::new(1, 0), Direction::East));

        let result = grid.neighbor(&Point::new(0, 0), &Direction::South);
        assert!(result.is_some(), "result: {:?}", result);
        assert_eq!(result.unwrap(), (Point::new(0, 1), Direction::South));

        let result = grid.neighbor(&Point::new(0, 0), &Direction::North);
        assert!(result.is_none(), "result: {:?}", result);

        let result = grid.neighbor(&Point::new(0, 0), &Direction::West);
        assert!(result.is_none(), "result: {:?}", result);
    }

    #[test]
    fn test_get_value() {
        let lines = vec![
            "..#.S#".to_string(),
            "......".to_string(),
            "E#...#".to_string(),
        ];
        let grid = Parser::parse_lines_to_grid(lines).unwrap();

        assert_eq!(grid.get_value('S'), vec![Point { x: 4, y: 0 }]);
        assert_eq!(grid.get_value('E'), vec![Point { x: 0, y: 2 }]);
        assert_eq!(
            grid.get_value('#'),
            vec![
                Point { x: 2, y: 0 },
                Point { x: 5, y: 0 },
                Point { x: 1, y: 2 },
                Point { x: 5, y: 2 }
            ]
        );
    }

    #[test]
    fn test_fill() {
        let mut grid = Grid::new_with(2, 3, |_| '.').unwrap();

        assert_eq!(grid[Point { x: 0, y: 0 }], '.');
        assert_eq!(grid[Point { x: 1, y: 0 }], '.');
        assert_eq!(grid[Point { x: 2, y: 0 }], '.');
        assert_eq!(grid[Point { x: 0, y: 1 }], '.');
        assert_eq!(grid[Point { x: 1, y: 1 }], '.');
        assert_eq!(grid[Point { x: 2, y: 1 }], '.');

        let result = grid.fill(&[(Point { x: 0, y: 0 }, 'S'), (Point { x: 2, y: 1 }, 'E')]);

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(grid[Point { x: 0, y: 0 }], 'S');
        assert_eq!(grid[Point { x: 1, y: 0 }], '.');
        assert_eq!(grid[Point { x: 2, y: 0 }], '.');
        assert_eq!(grid[Point { x: 0, y: 1 }], '.');
        assert_eq!(grid[Point { x: 1, y: 1 }], '.');
        assert_eq!(grid[Point { x: 2, y: 1 }], 'E');
    }

    #[test]
    fn test_fill_out_of_bounds() {
        let mut grid = Grid::new_with(2, 3, |_| '.').unwrap();

        assert_eq!(grid[Point { x: 0, y: 0 }], '.');
        assert_eq!(grid[Point { x: 1, y: 0 }], '.');
        assert_eq!(grid[Point { x: 2, y: 0 }], '.');
        assert_eq!(grid[Point { x: 0, y: 1 }], '.');
        assert_eq!(grid[Point { x: 1, y: 1 }], '.');
        assert_eq!(grid[Point { x: 2, y: 1 }], '.');

        let result = grid.fill(&[
            (Point { x: 0, y: 0 }, 'S'),
            (Point { x: 2, y: 1 }, 'E'),
            (Point { x: 42, y: 42 }, 'X'),
        ]);

        assert!(result.is_err(), "result: {:?}", result);
        assert_eq!(grid[Point { x: 0, y: 0 }], '.');
        assert_eq!(grid[Point { x: 1, y: 0 }], '.');
        assert_eq!(grid[Point { x: 2, y: 0 }], '.');
        assert_eq!(grid[Point { x: 0, y: 1 }], '.');
        assert_eq!(grid[Point { x: 1, y: 1 }], '.');
        assert_eq!(grid[Point { x: 2, y: 1 }], '.');
    }
}
