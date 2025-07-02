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
}
