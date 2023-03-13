use common::map::{Coord, Map, Path};
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use std::fmt::Debug;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input = Vec<Path>;

fn parse(input: &str) -> Input {
    let range = |s| {
        tuple((
            terminated(anychar, tag("=")),
            terminated(u32, tag("..")),
            u32,
        ))(s)
    };
    let point = |s| separated_pair(anychar, tag("="), u32)(s);

    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(separated_pair(point, tag(", "), range), |(point, range)| {
            Path::from_point_range(point, range)
        }),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Sand,
    Water,
    Clay,
    Flowing,
    Source,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sand => write!(f, "."),
            Self::Water => write!(f, "~"),
            Self::Clay => write!(f, "#"),
            Self::Flowing => write!(f, "|"),
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

struct CaveMap {
    map: Map<Tile>,
    source: Coord,

    min_y: usize,
    max_y: usize,
}

impl CaveMap {
    fn new(paths: &[Path]) -> Self {
        let mut min_y = usize::MAX;
        let mut max_y: usize = 0;
        let mut max_x: usize = 0;
        let mut min_x: usize = usize::MAX;

        // find the bounds of the map
        for path in paths {
            for &(x, y) in &path.segments {
                min_y = min_y.min(y);
                max_y = max_y.max(y);
                max_x = max_x.max(x);
                min_x = min_x.min(x);
            }
        }

        // we need some edge padding for a flow off the far left end of the input
        const EDGE_PADDING: usize = 1;
        min_x -= EDGE_PADDING;
        max_x += EDGE_PADDING;

        // for the purposes of this search, we need to ignore the min_y for the height because the source is above our min_y, but we have to bound the map
        let width = max_x - min_x;
        let height = max_y;

        // init the map with sand
        let tiles = vec![vec![Tile::Sand; width + 1]; height + 1];
        let mut map = Map::new(tiles);

        // place all the clay
        for path in paths {
            for point in path.all_points() {
                map.set(point.translate(min_x), Tile::Clay);
            }
        }

        // place the source
        let source = (500, 0).translate(min_x);
        map.set(source, Tile::Source);

        Self {
            map,
            source,
            min_y,
            max_y,
        }
    }

    fn simulate_water(&mut self, (x, y): Coord, spread: Spread) -> Option<usize> {
        // check if we're off the left or right edge or below the lowest rock
        if y == self.map.height {
            return None;
        }

        // okay now that we're here, simulate falling from here
        let tile = *self.map.get((x, y)).data;
        match tile {
            Tile::Clay | Tile::Water => Some(x),
            Tile::Flowing => None,
            Tile::Source | Tile::Sand => {
                if tile != Tile::Source {
                    self.map.set((x, y), Tile::Flowing);
                }

                // water always moves down when possible
                self.simulate_water((x, y + 1), Spread::Both)?;

                // then go left or right
                match spread {
                    Spread::Left => self.simulate_water((x - 1, y), Spread::Left),
                    Spread::Right => self.simulate_water((x + 1, y), Spread::Right),
                    Spread::Both => {
                        let left = self.simulate_water((x - 1, y), Spread::Left);
                        let right = self.simulate_water((x + 1, y), Spread::Right);

                        if let (Some(left), Some(right)) = (left, right) {
                            // there are walls on both sides, so make all this water
                            for x in left + 1..right {
                                self.map.set((x, y), Tile::Water);
                            }
                            Some(x)
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}

enum Spread {
    Left,
    Right,
    Both,
}

fn problem(input: &Input) -> (usize, usize) {
    let mut map = CaveMap::new(input);
    map.simulate_water(map.source, Spread::Both);

    let bounded: Vec<Tile> = map
        .map
        .into_iter()
        .filter_map(|x| (map.min_y <= x.coords.1 && x.coords.1 <= map.max_y).then_some(*x.data))
        .collect();

    let flowing = bounded.iter().filter(|x| *x == &Tile::Flowing).count();
    let still = bounded.iter().filter(|x| *x == &Tile::Water).count();

    (still + flowing, still)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (result1, result2) = problem(&input);
        assert_eq!(result1, 57);
        assert_eq!(result2, 29);
    }
}
