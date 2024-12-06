use std::iter;

use super::{direction::CardinalDirection, Coord, Grid, GridSquare};

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

pub trait HasNeighbors<T> {
    fn neighbors(&self, c: Coord) -> Neighbors<T>;
    fn all_neighbors(&self, c: Coord) -> AllNeighbors<T>;
}

impl<T> HasNeighbors<T> for Grid<T> {
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

pub struct Neighbors<'a, T> {
    pub north: Option<GridSquare<'a, T>>,
    pub south: Option<GridSquare<'a, T>>,
    pub east: Option<GridSquare<'a, T>>,
    pub west: Option<GridSquare<'a, T>>,
}

impl<'a, T> Neighbors<'a, T> {
    pub fn get(&self, direction: CardinalDirection) -> &Option<GridSquare<T>> {
        match direction {
            CardinalDirection::North => &self.north,
            CardinalDirection::South => &self.south,
            CardinalDirection::East => &self.east,
            CardinalDirection::West => &self.west,
        }
    }

    /// Iterate the orthogonal neighbors. Includes neighbors that are "off the grid" as `None`
    pub fn iter_all(&self) -> impl Iterator<Item = &Option<GridSquare<'a, T>>> {
        iter::once(&self.north)
            .chain(iter::once(&self.west))
            .chain(iter::once(&self.east))
            .chain(iter::once(&self.south))
    }

    /// Iterate the orthogonal neighbors. Only includes neighbors that exist
    pub fn iter(&self) -> impl Iterator<Item = &GridSquare<'a, T>> {
        self.iter_all().flatten()
    }
}

pub struct AllNeighbors<'a, T> {
    pub north: Option<GridSquare<'a, T>>,
    pub south: Option<GridSquare<'a, T>>,
    pub east: Option<GridSquare<'a, T>>,
    pub west: Option<GridSquare<'a, T>>,

    pub north_east: Option<GridSquare<'a, T>>,
    pub south_east: Option<GridSquare<'a, T>>,
    pub north_west: Option<GridSquare<'a, T>>,
    pub south_west: Option<GridSquare<'a, T>>,
}

impl<'a, T> AllNeighbors<'a, T> {
    /// Iterate the neighbors. Includes neighbors that are "off the grid" as `None`
    pub fn iter_all(&self) -> impl Iterator<Item = &Option<GridSquare<'a, T>>> {
        iter::once(&self.north_west)
            .chain(iter::once(&self.north))
            .chain(iter::once(&self.north_east))
            .chain(iter::once(&self.west))
            .chain(iter::once(&self.east))
            .chain(iter::once(&self.south_west))
            .chain(iter::once(&self.south))
            .chain(iter::once(&self.south_east))
    }

    /// Iterate the neighbors. Only includes neighbors that exist
    pub fn iter(&self) -> impl Iterator<Item = &GridSquare<'a, T>> {
        self.iter_all().flatten()
    }
}
