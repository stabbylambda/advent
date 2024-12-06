use std::fmt::{Debug, Display};

pub mod direction;
pub mod neighbors;
pub mod orthogonal;
pub mod path;

use crate::extensions::vecvec::VecVec;
pub use direction::CardinalDirection;
pub use neighbors::{AllNeighbors, Direction, HasNeighbors, Neighbors};
pub use path::Path;

#[derive(Clone)]
pub struct Grid<T> {
    pub points: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}
pub type Coord = (usize, usize);

impl<T> Grid<T> {
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
        self.get_opt_from_grid_index(grid_index).unwrap()
    }

    pub fn get_opt_from_grid_index(&self, grid_index: usize) -> Option<GridSquare<T>> {
        let y = grid_index / self.width;
        let x = grid_index % self.width;

        self.get_opt((x, y))
    }

    pub fn get_opt(&self, (x, y): Coord) -> Option<GridSquare<T>> {
        self.points.get(y).and_then(|row| {
            row.get(x).map(|data| GridSquare {
                map: self,
                coords: (x, y),
                data,
            })
        })
    }

    pub fn get_neighbor(&self, c: Coord, dir: CardinalDirection) -> Option<GridSquare<T>> {
        self.get_opt(c).and_then(|x| x.get_neighbor(dir))
    }

    pub fn get(&self, c: Coord) -> GridSquare<T> {
        self.get_opt(c).unwrap()
    }

    pub fn get_grid_index(&self, (x, y): Coord) -> usize {
        y * self.width + x
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

    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            index: 0,
            grid: self,
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn rotate(&self) -> Grid<T> {
        Grid::new(self.points.rotate())
    }

    pub fn transpose(&self) -> Grid<T> {
        Grid::new(self.points.transpose())
    }
}

pub struct GridIter<'a, T: 'a> {
    index: usize,
    grid: &'a Grid<T>,
}

impl<'a, T: 'a> Iterator for GridIter<'a, T> {
    type Item = GridSquare<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.grid.get_opt_from_grid_index(self.index);
        self.index += 1;
        next
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
pub struct GridSquare<'a, T> {
    map: &'a Grid<T>,
    pub coords: Coord,
    pub data: &'a T,
}

impl<'a, T> GridSquare<'a, T> {
    pub fn get_neighbor(&self, dir: CardinalDirection) -> Option<GridSquare<'a, T>> {
        let neighbors = self.neighbors();
        match dir {
            CardinalDirection::North => neighbors.north,
            CardinalDirection::South => neighbors.south,
            CardinalDirection::East => neighbors.east,
            CardinalDirection::West => neighbors.west,
        }
    }
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
