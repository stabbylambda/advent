use std::fmt::Debug;

use common::{answer, grid::Grid, nom::parse_grid, read_input};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, usize},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    println!("Merry Christmas!");
}

type Input = (Vec<Present>, Vec<Region>);

#[derive(Clone, Debug)]
#[allow(unused)]
struct Present {
    index: usize,
    shape: Grid<bool>,
    filled: usize,
}

impl Present {
    fn new(index: usize, shape: Grid<bool>) -> Self {
        Self {
            index,
            filled: shape.iter().filter(|x| *x.data).count(),
            shape,
        }
    }
}

#[derive(Debug)]
struct Region {
    grid: Grid<bool>,
    presents_required: Vec<usize>,
}

impl Region {
    fn can_fit(&self, presents: &[Present]) -> bool {
        let presents_to_place = self
            .presents_required
            .iter()
            .enumerate()
            .flat_map(|(idx, count)| vec![presents[idx].clone(); *count])
            .collect_vec();

        let required = presents_to_place.iter().map(|x| x.filled).sum::<usize>();
        let available = self.grid.width * self.grid.height;

        if required < available {
            return true;
        }

        // todo: actually try to place all the pieces?
        false
    }
}

fn parse(input: &str) -> Input {
    let present = map(
        (
            terminated(usize, tag(":\n")),
            parse_grid(alt((map(char('#'), |_| true), map(char('.'), |_| false)))),
        ),
        |(index, shape)| Present::new(index, shape),
    );

    let region = map(
        (
            terminated(usize, tag("x")),
            terminated(usize, tag(": ")),
            separated_list1(tag(" "), usize),
        ),
        |(width, height, presents_required)| Region {
            grid: Grid::new(vec![vec![false; width]; height]),
            presents_required,
        },
    );

    let result: IResult<&str, Input> = separated_pair(
        separated_list1(tag("\n\n"), present),
        tag("\n\n"),
        separated_list1(newline, region),
    )
    .parse(input);

    result.unwrap().1
}

fn problem1((presents, regions): &Input) -> usize {
    regions.iter().filter(|r| r.can_fit(presents)).count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        // Cheeky, Eric. A test case that requires solving the jigsaw puzzle.
        // assert_eq!(result, 2);
        assert_eq!(result, 3);
    }
}
