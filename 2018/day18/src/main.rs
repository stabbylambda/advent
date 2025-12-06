use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use common::{
    grid::{Grid, HasNeighbors},
    nom::parse_grid,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Tree,
    LumberYard,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Open => write!(f, "."),
            Tile::Tree => write!(f, "|"),
            Tile::LumberYard => write!(f, "#"),
        }
    }
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('.'), |_| Tile::Open),
        map(char('|'), |_| Tile::Tree),
        map(char('#'), |_| Tile::LumberYard),
    ))).parse(input);

    result.unwrap().1
}

fn tick(area: &Grid<Tile>) -> Grid<Tile> {
    let mut new_map = area.clone();
    for x in area.iter() {
        let neighbors = area.all_neighbors(x.coords);
        let tree_count = neighbors.iter().filter(|x| x.data == &Tile::Tree).count();
        let lumber_count = neighbors
            .iter()
            .filter(|x| x.data == &Tile::LumberYard)
            .count();

        let result = match x.data {
            // An open acre will become filled with trees if three or more adjacent acres contained trees.
            Tile::Open if tree_count >= 3 => Tile::Tree,
            Tile::Open => Tile::Open,

            // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
            Tile::Tree if lumber_count >= 3 => Tile::LumberYard,
            Tile::Tree => Tile::Tree,

            // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
            Tile::LumberYard if lumber_count >= 1 && tree_count >= 1 => Tile::LumberYard,
            Tile::LumberYard => Tile::Open,
        };

        new_map.set(x.coords, result);
    }

    new_map
}

fn score(area: &Grid<Tile>) -> usize {
    let tree_count = area.iter().filter(|x| x.data == &Tile::Tree).count();
    let lumber_count = area.iter().filter(|x| x.data == &Tile::LumberYard).count();

    tree_count * lumber_count
}

fn problem(input: &Input) -> (usize, usize) {
    let mut area = input.clone();
    let mut scores: HashMap<usize, usize> = HashMap::new();
    let mut previous_delta = 0;
    let mut answer_1 = 0;
    const TIMES: usize = 1_000_000_000;

    let mut current = 0;
    while current < TIMES {
        area = tick(&area);
        let resource_value = score(&area);

        // keep track of this one forever
        if current == 9 {
            answer_1 = resource_value;
        }

        // same old cycle detection code
        if let Some(previous) = scores.insert(resource_value, current) {
            let delta = current - previous;
            // did we reach a steady state?
            if delta == previous_delta {
                let cycle_size = previous.abs_diff(current);
                let cycle_count = (TIMES - current) / cycle_size;
                let new_index = current + (cycle_size * cycle_count);

                current = new_index;
            } else {
                previous_delta = delta;
            }
        }

        current += 1;
    }

    (answer_1, score(&area))
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (answer1, _answer2) = problem(&input);
        assert_eq!(answer1, 1147);
    }
}
