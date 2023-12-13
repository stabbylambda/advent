use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
};

use crate::transpose;

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<Coord>,
}

impl Path {
    pub fn new(segments: Vec<Coord>) -> Self {
        Self { segments }
    }

    pub fn from_point_range(
        (point_axis, point_value): (char, u32),
        (range_axis, range_start, range_end): (char, u32, u32),
    ) -> Path {
        let point_value = point_value as usize;
        let range_start = range_start as usize;
        let range_end = range_end as usize;

        Path::new(match (point_axis, range_axis) {
            ('x', 'y') => vec![(point_value, range_start), (point_value, range_end)],
            ('y', 'x') => vec![(range_start, point_value), (range_end, point_value)],
            _ => unreachable!(),
        })
    }

    pub fn all_points(&self) -> BTreeSet<Coord> {
        let mut points = BTreeSet::new();
        for pair in self.segments.windows(2) {
            let &[(x1, y1), (x2, y2)] = pair else {
                panic!()
            };

            let start_x = x1.min(x2);
            let end_x = x1.max(x2);

            let start_y = y1.min(y2);
            let end_y = y1.max(y2);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    points.insert((x, y));
                }
            }
        }

        points
    }
}

// This is my Map from last year
#[derive(Clone)]
pub struct Map<T> {
    pub points: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}
pub type Coord = (usize, usize);

pub struct Neighbors<'a, T: Copy> {
    pub north: Option<MapSquare<'a, T>>,
    pub south: Option<MapSquare<'a, T>>,
    pub east: Option<MapSquare<'a, T>>,
    pub west: Option<MapSquare<'a, T>>,
}

impl<'a, T: Copy> Neighbors<'a, T> {
    pub fn to_vec(&self) -> Vec<MapSquare<T>> {
        let v = vec![self.north, self.west, self.east, self.south];

        v.into_iter().flatten().collect()
    }
}

impl<'a, T: Copy> IntoIterator for &'a Neighbors<'a, T> {
    type Item = MapSquare<'a, T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

pub struct AllNeighbors<'a, T: Copy> {
    pub north: Option<MapSquare<'a, T>>,
    pub south: Option<MapSquare<'a, T>>,
    pub east: Option<MapSquare<'a, T>>,
    pub west: Option<MapSquare<'a, T>>,

    pub north_east: Option<MapSquare<'a, T>>,
    pub south_east: Option<MapSquare<'a, T>>,
    pub north_west: Option<MapSquare<'a, T>>,
    pub south_west: Option<MapSquare<'a, T>>,
}

impl<'a, T: Copy> AllNeighbors<'a, T> {
    pub fn to_vec(&self) -> Vec<MapSquare<T>> {
        let v = vec![
            self.north_west,
            self.north,
            self.north_east,
            self.west,
            self.east,
            self.south_west,
            self.south,
            self.south_east,
        ];

        v.into_iter().flatten().collect()
    }
}

impl<'a, T: Copy> IntoIterator for &'a AllNeighbors<'a, T> {
    type Item = MapSquare<'a, T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl<T: Copy> Map<T> {
    pub fn new(points: Vec<Vec<T>>) -> Map<T> {
        let height = points.len();
        let width = points[0].len();
        Map {
            points,
            height,
            width,
        }
    }

    pub fn set(&mut self, (x, y): Coord, data: T) {
        self.points[y][x] = data;
    }

    pub fn get_from_grid_index(&self, grid_index: usize) -> MapSquare<T> {
        let y = grid_index / self.width;
        let x = grid_index % self.width;

        self.get((x, y))
    }

    pub fn get(&self, (x, y): Coord) -> MapSquare<T> {
        let data = &self.points[y][x];
        MapSquare {
            map: self,
            coords: (x, y),
            data,
        }
    }

    pub fn neighbors(&self, (x, y): Coord) -> Neighbors<T> {
        let north = (y != 0).then(|| self.get((x, y - 1)));
        let south = (y != self.height - 1).then(|| self.get((x, y + 1)));
        let west = (x != 0).then(|| self.get((x - 1, y)));
        let east = (x != self.width - 1).then(|| self.get((x + 1, y)));

        Neighbors {
            north,
            south,
            east,
            west,
        }
    }

    pub fn all_neighbors(&self, (x, y): Coord) -> AllNeighbors<T> {
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

    pub fn get_grid_index(&self, (x, y): Coord) -> usize {
        y * self.width + x
    }

    pub fn transpose(&self) -> Map<T> {
        Map::new(transpose(&self.points))
    }

    pub fn print<F>(&self, f: F)
    where
        F: Fn(MapSquare<T>) -> char,
    {
        for (y, row) in self.points.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let square = MapSquare {
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

impl<'a, T: Copy> IntoIterator for &'a Map<T> {
    type Item = MapSquare<'a, T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width)
                    .map(|x| self.get((x, y)))
                    .collect::<Vec<MapSquare<'a, T>>>()
            })
            .collect::<Vec<MapSquare<'a, T>>>()
            .into_iter()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MapSquare<'a, T: Copy> {
    map: &'a Map<T>,
    pub coords: Coord,
    pub data: &'a T,
}

impl<'a, T: Copy> MapSquare<'a, T> {
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

impl<T> Debug for Map<T>
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

impl<T> Display for Map<T>
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
