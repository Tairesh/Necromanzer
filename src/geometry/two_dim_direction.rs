use std::convert::TryFrom;

use super::Direction;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum TwoDimDirection {
    East,
    West,
}

#[derive(Debug)]
pub enum ConvertError {
    North,
    South,
    Here,
}

impl TryFrom<Direction> for TwoDimDirection {
    type Error = ConvertError;

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::NorthEast | Direction::East | Direction::SouthEast => {
                Ok(TwoDimDirection::East)
            }
            Direction::SouthWest | Direction::West | Direction::NorthWest => {
                Ok(TwoDimDirection::West)
            }
            Direction::North => Err(ConvertError::North),
            Direction::South => Err(ConvertError::South),
            Direction::Here => Err(ConvertError::Here),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use super::{ConvertError, Direction, TwoDimDirection};

    #[test]
    fn to_two_dim() {
        let dir: TwoDimDirection = Direction::SouthEast.try_into().unwrap();
        assert!(matches!(dir, TwoDimDirection::East));
        let dir: TwoDimDirection = Direction::West.try_into().unwrap();
        assert!(matches!(dir, TwoDimDirection::West));
        let dir = TwoDimDirection::try_from(Direction::North);
        assert!(matches!(dir, Err(ConvertError::North)));
    }
}
