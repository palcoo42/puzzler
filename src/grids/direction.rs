#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    /// Constant to name the usual directions in a grid
    pub const CARDINAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn left(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            _ => panic!("Invalid direction for left '{:?}'", self),
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            _ => panic!("Invalid direction for right '{:?}'", self),
        }
    }

    pub fn backward(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            _ => panic!("Invalid direction for left '{:?}'", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left() {
        assert_eq!(Direction::North.left(), Direction::West);
        assert_eq!(Direction::East.left(), Direction::North);
        assert_eq!(Direction::South.left(), Direction::East);
        assert_eq!(Direction::West.left(), Direction::South);
    }

    #[test]
    fn test_right() {
        assert_eq!(Direction::North.right(), Direction::East);
        assert_eq!(Direction::East.right(), Direction::South);
        assert_eq!(Direction::South.right(), Direction::West);
        assert_eq!(Direction::West.right(), Direction::North);
    }

    #[test]
    fn test_backward() {
        assert_eq!(Direction::North.backward(), Direction::South);
        assert_eq!(Direction::East.backward(), Direction::West);
        assert_eq!(Direction::South.backward(), Direction::North);
        assert_eq!(Direction::West.backward(), Direction::East);
    }
}
