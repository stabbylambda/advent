use super::{CardinalDirection, Coord};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub Coord, pub CardinalDirection);

impl Position {
    pub fn new(c: Coord, dir: CardinalDirection) -> Self {
        Position(c, dir)
    }
    pub fn turn_right(&self) -> Self {
        let &Position((x, y), d) = self;
        let d = d.turn_right();
        Position((x, y), d)
    }

    pub fn turn_left(&self) -> Self {
        let &Position((x, y), d) = self;
        let d = d.turn_left();
        Position((x, y), d)
    }

    pub fn step(&self) -> Self {
        let &Position((x, y), d) = self;
        let c = match d {
            CardinalDirection::North => (x, y - 1),
            CardinalDirection::South => (x, y + 1),
            CardinalDirection::East => (x + 1, y),
            CardinalDirection::West => (x - 1, y),
        };

        Position(c, d)
    }
}
