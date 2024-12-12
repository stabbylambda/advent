use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}
impl CardinalDirection {
    pub fn turn_left(&self) -> CardinalDirection {
        *self - 90
    }

    pub fn turn_right(&self) -> CardinalDirection {
        *self + 90
    }
}

impl From<CardinalDirection> for i32 {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => 0,
            CardinalDirection::East => 90,
            CardinalDirection::South => 180,
            CardinalDirection::West => 270,
        }
    }
}

impl From<i32> for CardinalDirection {
    fn from(value: i32) -> Self {
        let value = if value < 0 { 360 + value } else { value };

        match value % 360 {
            0 => CardinalDirection::North,
            90 => CardinalDirection::East,
            180 => CardinalDirection::South,
            270 => CardinalDirection::West,
            _ => unreachable!("The number {value} is not a valid heading"),
        }
    }
}

impl AddAssign<i32> for CardinalDirection {
    fn add_assign(&mut self, rhs: i32) {
        *self = self.add(rhs)
    }
}
impl Add<i32> for CardinalDirection {
    type Output = CardinalDirection;

    fn add(self, rhs: i32) -> Self::Output {
        let current: i32 = self.into();
        let new = current + rhs;
        new.into()
    }
}

impl Sub<i32> for CardinalDirection {
    type Output = CardinalDirection;

    fn sub(self, rhs: i32) -> Self::Output {
        let current: i32 = self.into();
        let new = current - rhs;
        new.into()
    }
}

impl SubAssign<i32> for CardinalDirection {
    fn sub_assign(&mut self, rhs: i32) {
        *self = self.sub(rhs)
    }
}
