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
}
