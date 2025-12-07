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

fn problem2(grid: &Input) -> usize {
    let mut cache: BTreeMap<(usize, usize), usize> = BTreeMap::from_iter(grid.iter().map(|x| {
        (
            x.coords,
            match x.data {
                Tile::Start => 1,
                _ => 0,
            },
        )
    }));

    for x in grid.iter() {
        let &count = cache.get(&x.coords).unwrap_or(&0);
        if count == 0 {
            continue;
        }

        let Some(south) = x.get_neighbor(CardinalDirection::South) else {
            continue;
        };

        match south.data {
            Tile::Splitter => {
                if let Some(east) = south.get_neighbor(CardinalDirection::East) {
                    cache.entry(east.coords).and_modify(|x| *x += count);
                }

                if let Some(west) = south.get_neighbor(CardinalDirection::West) {
                    cache.entry(west.coords).and_modify(|x| *x += count);
                }
            }
            _ => {
                cache.entry(south.coords).and_modify(|x| *x += count);
            }
        }
    }

    cache
        .iter()
        .filter_map(|(&(_, y), &c)| (y == grid.height - 1).then_some(c))
        .sum()
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
        assert_eq!(result, 40)
    }
}
