use std::fmt::Debug;

// This is my Map from last year
pub struct Map<T> {
    pub points: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}
pub type Coord = (usize, usize);

impl<T> Map<T> {
    pub fn new(points: Vec<Vec<T>>) -> Map<T> {
        let height = points.len();
        let width = points[0].len();
        Map {
            points,
            height,
            width,
        }
    }

    pub fn get(&self, (x, y): Coord) -> MapSquare<T> {
        let data = &self.points[y][x];
        MapSquare {
            map: self,
            coords: (x, y),
            data,
        }
    }

    pub fn neighbors(&self, (x, y): Coord) -> Vec<MapSquare<T>> {
        let mut v = Vec::new();
        if y != 0 {
            v.push(self.get((x, y - 1)));
        }

        if x != 0 {
            v.push(self.get((x - 1, y)));
        }

        if y != self.height - 1 {
            v.push(self.get((x, y + 1)));
        }

        if x != self.width - 1 {
            v.push(self.get((x + 1, y)));
        }
        v
    }
}

impl<'a, T> IntoIterator for &'a Map<T> {
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

#[derive(Debug)]
pub struct MapSquare<'a, T> {
    map: &'a Map<T>,
    pub coords: Coord,
    pub data: &'a T,
}

impl<'a, T> MapSquare<'a, T> {
    pub fn neighbors(&self) -> Vec<MapSquare<'a, T>> {
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
