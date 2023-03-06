use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitType {
    Elf,
    Goblin,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    pub id: usize,
    pub location: Point,
    pub health: u32,
    pub power: u32,
    pub side: UnitType,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{}({})", self.side, self.id, self.health)
    }
}

impl Unit {
    pub fn new(id: usize, location: Point, side: UnitType) -> Self {
        Unit {
            id,
            location,
            health: 200,
            power: 3,
            side,
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage)
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}
pub type Point = (usize, usize);

pub fn reading_neighbors((y, x): Point) -> Vec<Option<Point>> {
    let north = (y != 0).then_some((y - 1, x));
    let east = Some((y, x + 1));
    let west = (x != 0).then_some((y, x - 1));
    let south = Some((y + 1, x));

    vec![north, west, east, south]
}
