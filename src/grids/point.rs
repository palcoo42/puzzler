use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn neighbor(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::NorthEast => Self::new(self.x + 1, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::SouthEast => Self::new(self.x + 1, self.y + 1),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::SouthWest => Self::new(self.x - 1, self.y + 1),
            Direction::West => Self::new(self.x - 1, self.y),
            Direction::NorthWest => Self::new(self.x - 1, self.y - 1),
        }
    }

    pub fn neighbor_at(&self, direction: &Direction, distance: isize) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - distance),
            Direction::NorthEast => Self::new(self.x + distance, self.y - distance),
            Direction::East => Self::new(self.x + distance, self.y),
            Direction::SouthEast => Self::new(self.x + distance, self.y + distance),
            Direction::South => Self::new(self.x, self.y + distance),
            Direction::SouthWest => Self::new(self.x - distance, self.y + distance),
            Direction::West => Self::new(self.x - distance, self.y),
            Direction::NorthWest => Self::new(self.x - distance, self.y - distance),
        }
    }

    pub fn neighbor_at_path(&self, direction: &Direction, distance: isize) -> Vec<Self> {
        match direction {
            Direction::North => (self.y - distance..self.y)
                .map(|y| Self::new(self.x, y))
                .rev()
                .collect::<Vec<_>>(),
            Direction::East => (self.x + 1..=self.x + distance)
                .map(|x| Self::new(x, self.y))
                .collect::<Vec<_>>(),
            Direction::South => (self.y + 1..=self.y + distance)
                .map(|y| Self::new(self.x, y))
                .collect::<Vec<_>>(),
            Direction::West => (self.x - distance..self.x)
                .map(|x| Self::new(x, self.y))
                .rev()
                .collect::<Vec<_>>(),
            d => panic!("{d:?} is not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbor() {
        let point = Point { x: 5, y: 3 };

        assert_eq!(point.neighbor(&Direction::North), Point { x: 5, y: 2 });
        assert_eq!(point.neighbor(&Direction::NorthEast), Point { x: 6, y: 2 });
        assert_eq!(point.neighbor(&Direction::East), Point { x: 6, y: 3 });
        assert_eq!(point.neighbor(&Direction::SouthEast), Point { x: 6, y: 4 });
        assert_eq!(point.neighbor(&Direction::South), Point { x: 5, y: 4 });
        assert_eq!(point.neighbor(&Direction::SouthWest), Point { x: 4, y: 4 });
        assert_eq!(point.neighbor(&Direction::West), Point { x: 4, y: 3 });
        assert_eq!(point.neighbor(&Direction::NorthWest), Point { x: 4, y: 2 });
    }

    #[test]
    fn test_neighbor_at() {
        let point = Point { x: 5, y: 3 };

        assert_eq!(
            point.neighbor_at(&Direction::North, 7),
            Point { x: 5, y: -4 }
        );
        assert_eq!(
            point.neighbor_at(&Direction::NorthEast, 7),
            Point { x: 12, y: -4 }
        );
        assert_eq!(
            point.neighbor_at(&Direction::East, 7),
            Point { x: 12, y: 3 }
        );
        assert_eq!(
            point.neighbor_at(&Direction::SouthEast, 7),
            Point { x: 12, y: 10 }
        );
        assert_eq!(
            point.neighbor_at(&Direction::South, 7),
            Point { x: 5, y: 10 }
        );
        assert_eq!(
            point.neighbor_at(&Direction::SouthWest, 7),
            Point { x: -2, y: 10 }
        );
        assert_eq!(
            point.neighbor_at(&Direction::West, 7),
            Point { x: -2, y: 3 }
        );
        assert_eq!(
            point.neighbor_at(&Direction::NorthWest, 7),
            Point { x: -2, y: -4 }
        );
    }

    #[test]
    fn test_neighbor_at_path() {
        let point = Point { x: 5, y: 3 };

        assert_eq!(
            point.neighbor_at_path(&Direction::North, 7),
            vec![
                Point { x: 5, y: 2 },
                Point { x: 5, y: 1 },
                Point { x: 5, y: 0 },
                Point { x: 5, y: -1 },
                Point { x: 5, y: -2 },
                Point { x: 5, y: -3 },
                Point { x: 5, y: -4 }
            ]
        );
        assert_eq!(
            point.neighbor_at_path(&Direction::East, 7),
            vec![
                Point { x: 6, y: 3 },
                Point { x: 7, y: 3 },
                Point { x: 8, y: 3 },
                Point { x: 9, y: 3 },
                Point { x: 10, y: 3 },
                Point { x: 11, y: 3 },
                Point { x: 12, y: 3 },
            ]
        );
        assert_eq!(
            point.neighbor_at_path(&Direction::South, 7),
            vec![
                Point { x: 5, y: 4 },
                Point { x: 5, y: 5 },
                Point { x: 5, y: 6 },
                Point { x: 5, y: 7 },
                Point { x: 5, y: 8 },
                Point { x: 5, y: 9 },
                Point { x: 5, y: 10 },
            ]
        );
        assert_eq!(
            point.neighbor_at_path(&Direction::West, 7),
            vec![
                Point { x: 4, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 2, y: 3 },
                Point { x: 1, y: 3 },
                Point { x: 0, y: 3 },
                Point { x: -1, y: 3 },
                Point { x: -2, y: 3 },
            ]
        );
    }
}
