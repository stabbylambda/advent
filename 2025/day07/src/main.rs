use std::collections::{BTreeMap, BTreeSet, VecDeque};

use common::{
    answer,
    grid::{CardinalDirection, Grid},
    nom::parse_grid,
    read_input,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<Tile>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Tile {
    Start,
    Splitter,
    Empty,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('S'), |_| Tile::Start),
        map(char('.'), |_| Tile::Empty),
        map(char('^'), |_| Tile::Splitter),
    )))
    .parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> usize {
    let start = x.iter().find(|x| x.data == &Tile::Start).unwrap();

    let mut splits: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let Some(next) = current.get_neighbor(CardinalDirection::South) else {
            continue;
        };

        if splits.contains(&next.coords) {
            continue;
        }

        match next.data {
            Tile::Empty => queue.push_back(next),
            Tile::Splitter => {
                splits.insert(next.coords);

                if let Some(east) = next.get_neighbor(CardinalDirection::East) {
                    queue.push_back(east);
                }
                if let Some(west) = next.get_neighbor(CardinalDirection::West) {
                    queue.push_back(west);
                }
            }
            Tile::Start => unreachable!(),
        }
    }

    splits.len()
}

fn problem2(x: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
