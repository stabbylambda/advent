use common::grid::{Coord, Grid, Path};
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Air,
    Sand,
    Rock,
    Source,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Sand => write!(f, "o"),
            Self::Rock => write!(f, "#"),
            Self::Source => write!(f, "+"),
        }
    }
}
trait CaveCoord {
    fn translate(&self, min_x: usize) -> Coord;
}
impl CaveCoord for Coord {
    // normalize a point so that x is a value relative to the minimum x as the zero
    fn translate(&self, min_x: usize) -> Coord {
        let (x, y) = self;
        let new_x = x - min_x;
        (new_x, *y)
    }
}

pub struct CaveMap {
    pub map: Grid<Tile>,
    pub source: Coord,
}

impl CaveMap {
    pub fn new(paths: &[Path], has_floor: bool) -> Self {
        let min_y = 0;
        let mut max_y: usize = 0;
        let mut max_x: usize = 0;
        let mut min_x: usize = usize::MAX;

        // find the bounds of the map
        for path in paths {
            for &(x, y) in &path.segments {
                max_y = max_y.max(y);
                max_x = max_x.max(x);
                min_x = min_x.min(x);
            }
        }

        // the floor is 2 levels below our maximum y point, we also need to extend the x direction "infinitely"
        if has_floor {
            const EDGE_PADDING: usize = 200;

            min_x -= EDGE_PADDING;
            max_x += EDGE_PADDING;
            max_y += 2;
        }

        let width = max_x - min_x;
        let height = max_y - min_y;

        // init the map with air
        let tiles = vec![vec![Tile::Air; width + 1]; height + 1];
        let mut map = Grid::new(tiles);

        // place all the rocks
        for path in paths {
            for point in path.all_points() {
                let rock = point.translate(min_x);
                map.set(rock, Tile::Rock);
            }
        }

        // draw the floor across the entire last row
        if has_floor {
            for floor_x in 0..=width {
                map.set((floor_x, max_y), Tile::Rock);
            }
        }

        // place the source
        let source = (500, 0).translate(min_x);
        map.set(source, Tile::Source);

        Self { map, source }
    }
}
