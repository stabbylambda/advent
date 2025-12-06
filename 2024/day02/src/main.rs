use common::{answer, read_input};
use itertools::Itertools;
use nom::{
    character::complete::{char, i32, newline},
    combinator::map,
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Report>;
fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, map(separated_list1(char(' '), i32), Report)).parse(input);

    result.unwrap().1
}

struct Report(Vec<i32>);

impl Report {
    fn is_safe(&self) -> bool {
        let deltas = self
            .0
            .iter()
            .tuple_windows()
            .map(|(a, b)| a - b)
            .collect_vec();

        let monotonic = deltas.iter().all(|x| *x > 0) || deltas.iter().all(|x| *x < 0);
        let in_range = deltas.iter().all(|x| x.abs() <= 3);

        monotonic && in_range
    }

    fn without_index(&self, i: usize) -> Self {
        // make a clone and drop the index
        let mut v = self.0.clone();
        v.remove(i);
        Self(v)
    }

    fn is_safe_with_dampener(&self) -> bool {
        // if we're already safe, just bail
        if self.is_safe() {
            return true;
        }

        // otherwise remove each element and find out if we're safe without it (there's probably some more optimal
        // way, but who cares)
        (0..self.0.len())
            .map(|i| self.without_index(i))
            .any(|r| r.is_safe())
    }
}

fn problem1(input: &Input) -> usize {
    input.iter().filter(|x| x.is_safe()).count()
}

fn problem2(input: &Input) -> usize {
    input.iter().filter(|x| x.is_safe_with_dampener()).count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4)
    }
}
