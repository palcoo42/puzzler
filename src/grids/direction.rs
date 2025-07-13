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

    /// Constant to name the all directions in a grid
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub fn left(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            _ => panic!("Invalid direction for left '{self:?}'"),
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            _ => panic!("Invalid direction for right '{self:?}'"),
        }
    }

    pub fn backward(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            _ => panic!("Invalid direction for left '{self:?}'"),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::West),
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Direction::try_from(value as char)
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

    #[test]
    fn test_try_from_char() {
        assert_eq!(Direction::try_from('<'), Ok(Direction::West));
        assert_eq!(Direction::try_from('^'), Ok(Direction::North));
        assert_eq!(Direction::try_from('>'), Ok(Direction::East));
        assert_eq!(Direction::try_from('v'), Ok(Direction::South));
    }

    #[test]
    fn test_try_from_char_error() {
        assert_eq!(Direction::try_from('a'), Err(()));
        assert_eq!(Direction::try_from('/'), Err(()));
        assert_eq!(Direction::try_from('x'), Err(()));
        assert_eq!(Direction::try_from('l'), Err(()));
    }

    #[test]
    fn test_try_from_u8() {
        assert_eq!(Direction::try_from(b'<'), Ok(Direction::West));
        assert_eq!(Direction::try_from(b'^'), Ok(Direction::North));
        assert_eq!(Direction::try_from(b'>'), Ok(Direction::East));
        assert_eq!(Direction::try_from(b'v'), Ok(Direction::South));
    }

    #[test]
    fn test_try_from_u8_error() {
        assert_eq!(Direction::try_from(b'a'), Err(()));
        assert_eq!(Direction::try_from(b'/'), Err(()));
        assert_eq!(Direction::try_from(b'x'), Err(()));
        assert_eq!(Direction::try_from(b'l'), Err(()));
    }
}
