use common::dijkstra::{connected_components, Edge};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input = Vec<Vec<Edge>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        preceded(
            terminated(u32, tag(" <-> ")),
            separated_list1(tag(", "), map(u32, |x| Edge::new(x as usize))),
        ),
    )(input);

    result.unwrap().1
}

fn problem(input: &Input) -> (usize, usize) {
    let connected = connected_components(input);
    let zero_group_len = connected[&0].len();
    let total = connected.len();

    (zero_group_len, total)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (zero_len, total) = problem(&input);
        assert_eq!(zero_len, 6);
        assert_eq!(total, 2);
    }
}
