use common::{answer, grid::Grid, read_input};
use itertools::Itertools;
use nom::{
    bytes::tag,
    character::complete::{newline, usize},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<(usize, usize)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_pair(usize, tag(","), usize)).parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> usize {
    x.iter()
        .combinations(2)
        .map(|x| {
            let (ax, ay) = *x[0];
            let (bx, by) = *x[1];

            let x = ax.abs_diff(bx) + 1;
            let y = ay.abs_diff(by) + 1;

            x * y
        })
        .max()
        .unwrap()
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
        assert_eq!(result, 50);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
