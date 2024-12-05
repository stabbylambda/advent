use crate::grid::{Grid, GridSquare};

#[derive(Debug)]
pub struct OrthogonalNeighbors<'a, T> {
    pub north: Vec<GridSquare<'a, T>>,
    pub south: Vec<GridSquare<'a, T>>,
    pub east: Vec<GridSquare<'a, T>>,
    pub west: Vec<GridSquare<'a, T>>,
}

pub trait Orthogonal<'a, T> {
    fn orthogonal_neighbors(&'a self, square: &GridSquare<T>) -> OrthogonalNeighbors<'a, T>;
}

impl<'a, T> Orthogonal<'a, T> for Grid<T> {
    fn orthogonal_neighbors(&'a self, square: &GridSquare<T>) -> OrthogonalNeighbors<'a, T> {
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
