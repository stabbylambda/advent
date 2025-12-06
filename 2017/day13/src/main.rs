use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<(u32, u32)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_pair(u32, tag(": "), u32)).parse(input);

    result.unwrap().1
}

fn sensors_tripped(input: &Input, start: u32) -> Vec<(u32, u32)> {
    input
        .iter()
        .filter_map(|&(t, depth)| {
            let cycle = 2 * (depth - 1);
            (start + t).is_multiple_of(cycle).then_some((t, depth))
        })
        .collect()
}

fn problem1(input: &Input) -> u32 {
    sensors_tripped(input, 0).iter().map(|(x, y)| x * y).sum()
}

fn problem2(input: &Input) -> u32 {
    (0u32..)
        .find(|&n| sensors_tripped(input, n).is_empty())
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 24)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 10)
    }
}
