use common::dijkstra::{shortest_path, Edge};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<(usize, Vec<usize>)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(
            map(u32, |x| x as usize),
            tag(" <-> "),
            separated_list1(tag(", "), map(u32, |x| x as usize)),
        ),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let edges: Vec<Vec<Edge>> = input
        .iter()
        .map(|(_, v)| v.iter().map(|node| Edge::new(*node)).collect())
        .collect();

    input
        .iter()
        .filter_map(|(start, _)| shortest_path(&edges, *start, 0))
        .count()
}

fn problem2(_input: &Input) -> u32 {
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
        assert_eq!(result, 6)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
