use std::collections::{BTreeSet, VecDeque};

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
    let mut beams: Vec<usize> = grid.points[0]
        .iter()
        .map(|x| (x == &Tile::Start).into())
        .collect();
    let mut next_beams = beams.clone();

    for y in 0..grid.height {
        next_beams.fill(0);

        for (x, &count) in beams.iter().enumerate().filter(|(_, c)| **c != 0) {
            let Some(south) = grid.get_neighbor((x, y), CardinalDirection::South) else {
                continue;
            };

            match south.data {
                Tile::Splitter => {
                    next_beams[x - 1] += count;
                    next_beams[x + 1] += count;
                }
                _ => {
                    next_beams[x] += count;
                }
            }
        }

        std::mem::swap(&mut beams, &mut next_beams);
    }

    next_beams.iter().sum()
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
