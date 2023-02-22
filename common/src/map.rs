use std::fmt::Debug;

// This is my Map from last year
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
