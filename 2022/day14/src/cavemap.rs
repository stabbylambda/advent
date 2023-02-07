use common::map::{Coord, Map};
use std::{collections::BTreeSet, fmt::Debug};

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

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<Coord>,
}

impl Path {
    pub fn new(segments: Vec<Coord>) -> Self {
        Self { segments }
    }
    pub fn get_min_x(&self) -> usize {
        self.segments.iter().map(|&(x, _)| x).min().unwrap()
    }

    pub fn get_max_x(&self) -> usize {
        self.segments.iter().map(|&(x, _)| x).max().unwrap()
    }

    pub fn get_max_y(&self) -> usize {
        self.segments.iter().map(|&(_, y)| y).max().unwrap()
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
    pub map: Map<Tile>,
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
        let mut map = Map::new(tiles);

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
