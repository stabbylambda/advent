use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

impl Heading {
    pub fn turn_left(&self) -> Heading {
        *self - 90
    }

    pub fn turn_right(&self) -> Heading {
        *self + 90
    }
}

impl From<Heading> for i32 {
    fn from(value: Heading) -> Self {
        match value {
            Heading::North => 0,
            Heading::East => 90,
            Heading::South => 180,
            Heading::West => 270,
        }
    }
}

impl From<i32> for Heading {
    fn from(value: i32) -> Self {
        let value = if value < 0 { 360 + value } else { value };

        match value % 360 {
            0 => Heading::North,
            90 => Heading::East,
            180 => Heading::South,
            270 => Heading::West,
            _ => unreachable!("The number {value} is not a valid heading"),
        }
    }
}

impl AddAssign<i32> for Heading {
    fn add_assign(&mut self, rhs: i32) {
        *self = self.add(rhs)
    }
}
impl Add<i32> for Heading {
    type Output = Heading;

    fn add(self, rhs: i32) -> Self::Output {
        let current: i32 = self.into();
        let new = current + rhs;
        new.into()
    }
}

impl Sub<i32> for Heading {
    type Output = Heading;

    fn sub(self, rhs: i32) -> Self::Output {
        let current: i32 = self.into();
        let new = current - rhs;
        new.into()
    }
}

impl SubAssign<i32> for Heading {
    fn sub_assign(&mut self, rhs: i32) {
        *self = self.sub(rhs)
    }
}
