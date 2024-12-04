use std::fmt::{Debug, Display};

use crate::extensions::vecvec::VecVec;
pub mod neighbors;
pub mod path;
pub use neighbors::{AllNeighbors, CardinalDirection, Direction, HasNeighbors, Neighbors};
pub use path::Path;

// This is my Map from last year
#[derive(Clone)]
pub struct Grid<T> {
    pub points: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}
pub type Coord = (usize, usize);

impl<T: Copy> Grid<T> {
    pub fn new(points: Vec<Vec<T>>) -> Grid<T> {
        let height = points.len();
        let width = points[0].len();
        Grid {
            points,
            height,
            width,
        }
    }

    pub fn set(&mut self, (x, y): Coord, data: T) {
        self.points[y][x] = data;
    }

    pub fn get_from_grid_index(&self, grid_index: usize) -> GridSquare<T> {
        let y = grid_index / self.width;
        let x = grid_index % self.width;

        self.get((x, y))
    }

    pub fn get(&self, (x, y): Coord) -> GridSquare<T> {
        let data = &self.points[y][x];
        GridSquare {
            map: self,
            coords: (x, y),
            data,
        }
    }

    pub fn get_grid_index(&self, (x, y): Coord) -> usize {
        y * self.width + x
    }

    pub fn rotate(&self) -> Grid<T> {
        Grid::new(self.points.rotate())
    }

    pub fn transpose(&self) -> Grid<T> {
        Grid::new(self.points.transpose())
    }

    pub fn print<F>(&self, f: F)
    where
        F: Fn(GridSquare<T>) -> char,
    {
        for (y, row) in self.points.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let square = GridSquare {
                    map: self,
                    coords: (x, y),
                    data: col,
                };

                print!("{}", f(square));
            }
            println!();
        }
    }
}

impl<'a, T: Copy> IntoIterator for &'a Grid<T> {
    type Item = GridSquare<'a, T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width)
                    .map(|x| self.get((x, y)))
                    .collect::<Vec<GridSquare<'a, T>>>()
            })
            .collect::<Vec<GridSquare<'a, T>>>()
            .into_iter()
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.points[..] {
            for col in &row[..] {
                write!(f, "{col:?}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.points[..] {
            for col in &row[..] {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridSquare<'a, T: Copy> {
    map: &'a Grid<T>,
    pub coords: Coord,
    pub data: &'a T,
}

impl<'a, T: Copy> GridSquare<'a, T> {
    pub fn neighbors(&self) -> Neighbors<'a, T> {
        self.map.neighbors(self.coords)
    }

    pub fn all_neighbors(&self) -> AllNeighbors<'a, T> {
        self.map.all_neighbors(self.coords)
    }

    pub fn get_grid_index(&self) -> usize {
        let (x, y) = self.coords;
        y * self.map.width + x
    }
}
