use common::{answer, grid::Grid, nom::parse_grid, read_input};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{anychar, char},
    combinator::{map, verify},
    AsChar, IResult, Parser,
};
use std::ops::RangeInclusive;

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<Option<char>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('.'), |_| None),
        map(verify(anychar, |x| x.is_alphanum()), Some),
    ))).parse(input);

    result.unwrap().1
}

// only keep the ones in the bounds of the map
fn check_bounds(&(x, y): &(isize, isize), input: &Input) -> bool {
    x >= 0 && y >= 0 && x < input.width as isize && y < input.height as isize
}

fn count_antinodes(input: &Input, range: RangeInclusive<isize>) -> usize {
    let antenna_groups = input
        .iter()
        .filter_map(|x| {
            x.data
                .map(|c| (c, (x.coords.0 as isize, x.coords.1 as isize)))
        })
        .into_group_map();

    antenna_groups
        .iter()
        .flat_map(|(_, locations)| locations.iter().permutations(2))
        .map(|pair| (pair[0], pair[1]))
        .flat_map(|((ax, ay), (bx, by))| {
            let (delta_x, delta_y) = (bx - ax, by - ay);

            // generate all the antinodes along the line (in one direction,
            // the other will be handled by the other permutation)
            let result: Vec<_> = range
                .clone()
                .map(|mul| (ax - (mul * delta_x), ay - (mul * delta_y)))
                .filter(|c| check_bounds(c, input))
                .collect();

            result
        })
        .unique()
        .count()
}

fn problem1(input: &Input) -> usize {
    // only generate nodes 1 delta away from the antennas
    count_antinodes(input, 1..=1)
}

fn problem2(input: &Input) -> usize {
    // generate from 0 (the antenna) all the way out to a ridiculous multiple
    count_antinodes(input, 0..=(input.width as isize))
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 14)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 34)
    }
}
