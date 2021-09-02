#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum Direction {
    Here,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum TwoDimDirection {
    East,
    West,
}

impl Direction {
    pub fn dx(&self) -> i8 {
        match self {
            Direction::NorthWest | Direction::West | Direction::SouthWest => -1,
            Direction::NorthEast | Direction::East | Direction::SouthEast => 1,
            Direction::North | Direction::South | Direction::Here => 0,
        }
    }

    pub fn dy(&self) -> i8 {
        match self {
            Direction::NorthEast | Direction::North | Direction::NorthWest => -1,
            Direction::SouthEast | Direction::South | Direction::SouthWest => 1,
            Direction::East | Direction::West | Direction::Here => 0,
        }
    }

    pub fn as_two_dimensional(&self) -> Option<TwoDimDirection> {
        match self {
            Direction::NorthEast | Direction::East | Direction::SouthEast => {
                Some(TwoDimDirection::East)
            }
            Direction::SouthWest | Direction::West | Direction::NorthWest => {
                Some(TwoDimDirection::West)
            }
            Direction::North | Direction::South | Direction::Here => None,
        }
    }

    pub fn is_here(&self) -> bool {
        matches!(self, Direction::Here)
    }
}
