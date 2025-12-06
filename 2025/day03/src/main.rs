use common::{answer, nom::single_digit_u64, read_input};
use nom::{
    character::complete::newline,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Vec<u64>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, many1(single_digit_u64)).parse(input);

    result.unwrap().1
}

fn find_max(count: usize, remaining: &[u64]) -> u64 {
    fn _find_max(current: u64, count: usize, remaining: &[u64]) -> u64 {
        if remaining.is_empty() || count == 0 {
            return current;
        }

        let tail_count = remaining.len() - count + 1;
        let next = &remaining[..tail_count];

        let max = next.iter().max().unwrap();
        let next_idx = next.iter().position(|x| x == max).unwrap();

        let rest = &remaining[next_idx + 1..];

        _find_max(10 * current + max, count - 1, rest)
    }

    _find_max(0, count, remaining)
}

fn problem1(x: &Input) -> u64 {
    x.iter().map(|x| find_max(2, x)).sum()
}

fn problem2(x: &Input) -> u64 {
    x.iter().map(|x| find_max(12, x)).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 357);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 3121910778619)
    }
}
