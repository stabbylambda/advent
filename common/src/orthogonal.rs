use crate::map::{Map, MapSquare};

#[derive(Debug)]
pub struct OrthogonalNeighbors<'a, T> {
    pub north: Vec<MapSquare<'a, T>>,
    pub south: Vec<MapSquare<'a, T>>,
    pub east: Vec<MapSquare<'a, T>>,
    pub west: Vec<MapSquare<'a, T>>,
}

pub trait Orthogonal<'a, T> {
    fn orthogonal_neighbors(&'a self, square: &MapSquare<T>) -> OrthogonalNeighbors<'a, T>;
}

impl<'a, T> Orthogonal<'a, T> for Map<T> {
    fn orthogonal_neighbors(&'a self, square: &MapSquare<T>) -> OrthogonalNeighbors<'a, T> {
        let (x, y) = square.coords;

        // check the vertical and horizontal from this square
        let north = (0..y).rev().map(|dy| self.get((x, dy))).collect();
        let south = (y + 1..self.height).map(|dy| self.get((x, dy))).collect();
        let west = (0..x).rev().map(|dx| self.get((dx, y))).collect();
        let east = (x + 1..self.width).map(|dx| self.get((dx, y))).collect();

        OrthogonalNeighbors {
            north,
            south,
            east,
            west,
        }
    }
}
