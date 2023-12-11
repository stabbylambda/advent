use std::fmt::Debug;

use common::{
    extensions::PointExt,
    map::{Coord, Map},
};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Map<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((
                map(char('.'), |_| Tile::Empty),
                map(char('#'), |_| Tile::Galaxy),
            ))),
        ),
        Map::new,
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug)]
struct Universe {
    map: Map<Tile>,
    row_costs: Vec<usize>,
    column_costs: Vec<usize>,
}

impl Universe {
    fn new(map: Map<Tile>, expansion_amount: usize) -> Universe {
        // Expand any row that's all empty
        let row_costs = (0..map.height)
            .map(|y| {
                if map.points[y].iter().all(|t| *t == Tile::Empty) {
                    /* We have to subtract 1 because these are extra costs. The base cost comes from
                    the manhattan function itself */
                    expansion_amount - 1
                } else {
                    0
                }
            })
            .collect();

        // expand any column that's all empty
        let column_costs = (0..map.width)
            .map(|x| {
                if map.points.iter().all(|r| r[x] == Tile::Empty) {
                    /* We have to subtract 1 because these are extra costs. The base cost comes from
                    the manhattan function itself */
                    expansion_amount - 1
                } else {
                    0
                }
            })
            .collect();

        Universe {
            map,
            row_costs,
            column_costs,
        }
    }

    fn get_galaxy_coordinates(&self) -> Vec<(usize, usize)> {
        self.map
            .into_iter()
            .filter_map(|x| (*x.data == Tile::Galaxy).then_some(x.coords))
            .collect_vec()
    }

    /** Get the manhattan distance between two points that takes into account universe expansion */
    fn manhattan(&self, a: Coord, b: Coord) -> usize {
        let (ax, ay) = a;
        let (bx, by) = b;

        // first get the normal manhattan distance between the two points
        let basic_manhattan = a.manhattan(&b);

        // min/max the coordinates so we can get the ranges for the extra traversal costs
        let min_x = ax.min(bx);
        let max_x = ax.max(bx);
        let min_y = ay.min(by);
        let max_y = ay.max(by);

        // sum the extra costs to simulate universe expansion
        let extra_row: usize = (min_y..max_y).map(|y| self.row_costs[y]).sum();
        let extra_col: usize = (min_x..max_x).map(|x| self.column_costs[x]).sum();

        basic_manhattan + extra_col + extra_row
    }
}

fn get_distances(input: &Input, expansion_amount: usize) -> usize {
    let universe = Universe::new(input.clone(), expansion_amount);

    // find the manhattan distances between all the galaxies
    let distances: usize = universe
        .get_galaxy_coordinates()
        .into_iter()
        .permutations(2)
        .map(|x| universe.manhattan(x[0], x[1]))
        .sum();

    // permutations gives us all pairs twice...so rather than sorting pairs and de-duping...just divide by 2
    distances / 2
}

fn problem1(input: &Input) -> usize {
    get_distances(input, 2)
}

fn problem2(input: &Input) -> usize {
    get_distances(input, 1000000)
}

#[cfg(test)]
mod test {
    use crate::{get_distances, parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 374)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        assert_eq!(get_distances(&input, 10), 1030);
        assert_eq!(get_distances(&input, 100), 8410);
    }
}
