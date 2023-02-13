use std::collections::VecDeque;

use nom::{
    bytes::complete::tag, character::complete::u32, combinator::map, multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input, 256);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<usize>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(tag(","), map(u32, |x| x as usize))(input);

    result.unwrap().1
}

fn problem1(input: &Input, max: usize) -> usize {
    let mut rope: VecDeque<usize> = (0..max).collect();
    let mut current = 0;

    for (skip, length) in input.iter().enumerate() {
        /* Rotate to make the current at 0, reverse up to the length,
        then rotate back so we're in the right spot again */
        rope.rotate_left(current);
        rope.make_contiguous()[0..*length].reverse();
        rope.rotate_right(current);

        // Move the current position forward by that length plus the skip size.
        current = (current + length + skip) % max;
    }

    rope[0] * rope[1]
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
        let result = problem1(&input, 5);
        assert_eq!(result, 12)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
