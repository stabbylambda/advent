use super::{Coord, Grid, GridSquare};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

pub trait HasNeighbors<T: Copy> {
    fn neighbors(&self, c: Coord) -> Neighbors<T>;
    fn all_neighbors(&self, c: Coord) -> AllNeighbors<T>;
}

impl<T: Copy> HasNeighbors<T> for Grid<T> {
    fn neighbors(&self, (x, y): Coord) -> Neighbors<T> {
        let all = self.all_neighbors((x, y));

        Neighbors {
            north: all.north,
            south: all.south,
            east: all.east,
            west: all.west,
        }
    }

    fn all_neighbors(&self, (x, y): Coord) -> AllNeighbors<T> {
        let north = (y > 0).then(|| self.get((x, y - 1)));
        let south = (y < self.height - 1).then(|| self.get((x, y + 1)));
        let west = (x > 0).then(|| self.get((x - 1, y)));
        let east = (x < self.width - 1).then(|| self.get((x + 1, y)));

        let north_west = (y > 0 && x > 0).then(|| self.get((x - 1, y - 1)));
        let north_east = (y > 0 && x < self.width - 1).then(|| self.get((x + 1, y - 1)));

        let south_west = (y < self.height - 1 && x > 0).then(|| self.get((x - 1, y + 1)));
        let south_east =
            (y < self.height - 1 && x < self.width - 1).then(|| self.get((x + 1, y + 1)));

        AllNeighbors {
            north,
            north_east,
            north_west,
            south,
            south_east,
            south_west,
            east,
            west,
        }
    }
}

pub struct Neighbors<'a, T: Copy> {
    pub north: Option<GridSquare<'a, T>>,
    pub south: Option<GridSquare<'a, T>>,
    pub east: Option<GridSquare<'a, T>>,
    pub west: Option<GridSquare<'a, T>>,
}

impl<'a, T: Copy> Neighbors<'a, T> {
    pub fn get(&self, direction: CardinalDirection) -> Option<GridSquare<T>> {
        match direction {
            CardinalDirection::North => self.north,
            CardinalDirection::South => self.south,
            CardinalDirection::East => self.east,
            CardinalDirection::West => self.west,
        }
    }
}

impl<'a, T: Copy> Neighbors<'a, T> {
    pub fn to_vec(&self) -> Vec<GridSquare<T>> {
        let v = vec![self.north, self.west, self.east, self.south];

        v.into_iter().flatten().collect()
    }
}

impl<'a, T: Copy> IntoIterator for &'a Neighbors<'a, T> {
    type Item = GridSquare<'a, T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}
pub struct AllNeighbors<'a, T: Copy> {
    pub north: Option<GridSquare<'a, T>>,
    pub south: Option<GridSquare<'a, T>>,
    pub east: Option<GridSquare<'a, T>>,
    pub west: Option<GridSquare<'a, T>>,

    pub north_east: Option<GridSquare<'a, T>>,
    pub south_east: Option<GridSquare<'a, T>>,
    pub north_west: Option<GridSquare<'a, T>>,
    pub south_west: Option<GridSquare<'a, T>>,
}

impl<'a, T: Copy> AllNeighbors<'a, T> {
    pub fn get(&self, direction: Direction) -> Option<GridSquare<T>> {
        match direction {
            Direction::North => self.north,
            Direction::South => self.south,
            Direction::East => self.east,
            Direction::West => self.west,
            Direction::NorthWest => self.north_west,
            Direction::NorthEast => self.north_east,
            Direction::SouthWest => self.south_west,
            Direction::SouthEast => self.south_east,
        }
    }
    pub fn to_all_vec(&self) -> Vec<Option<GridSquare<T>>> {
        vec![
            self.north_west,
            self.north,
            self.north_east,
            self.west,
            self.east,
            self.south_west,
            self.south,
            self.south_east,
        ]
    }

    pub fn to_vec(&self) -> Vec<GridSquare<T>> {
        self.to_all_vec().into_iter().flatten().collect()
    }
}

impl<'a, T: Copy> IntoIterator for &'a AllNeighbors<'a, T> {
    type Item = GridSquare<'a, T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}
