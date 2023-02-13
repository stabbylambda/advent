use std::collections::VecDeque;

use nom::{
    bytes::complete::tag, character::complete::u32, combinator::map, multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");

    let score = problem1(input, 256);
    println!("problem 1 score: {score}");

    let score = problem2(input, 256);
    println!("problem 2 score: {score}");
}

fn parse(input: &str) -> Vec<usize> {
    let result: IResult<&str, Vec<usize>> =
        separated_list1(tag(","), map(u32, |x| x as usize))(input);

    result.unwrap().1
}

fn hash(input: Vec<usize>, max: usize, rounds: usize) -> impl SparseHash {
    let mut rope: VecDeque<usize> = (0..max).collect();
    let mut current = 0;
    let mut skip = 0;

    for _round in 0..rounds {
        for length in &input {
            /* Rotate to make the current at 0, reverse up to the length,
            then rotate back so we're in the right spot again */
            rope.rotate_left(current);
            rope.make_contiguous()[0..*length].reverse();
            rope.rotate_right(current);

            // Move the current position forward by that length plus the skip size.
            current = (current + length + skip) % max;
            skip += 1;
        }
    }

    rope.iter().cloned().collect::<Vec<usize>>()
}

trait SparseHash {
    fn check(&self) -> usize;
    fn to_dense(&self) -> String;
}

impl SparseHash for Vec<usize> {
    fn check(&self) -> usize {
        self[0] * self[1]
    }

    fn to_dense(&self) -> String {
        let dense: String = self
            .chunks(16)
            .map(|x| x.iter().fold(0, |a, b| a ^ b))
            .map(|c| format!("{:02x}", c))
            .collect();

        dense
    }
}

fn problem1(input: &str, max: usize) -> usize {
    let input = parse(input);
    let rope = hash(input, max, 1);
    rope.check()
}

fn problem2(input: &str, max: usize) -> String {
    let mut input: Vec<usize> = input.bytes().into_iter().map(|x| x as usize).collect();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);
    let sparse = hash(input, max, 64);

    sparse.to_dense()
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let result = problem1(input, 5);
        assert_eq!(result, 12)
    }

    #[test]
    fn second() {
        let cases = [
            ("", "a2582a3a0e66e6e86e3812dcb672a272"),
            ("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"),
            ("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"),
            ("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"),
        ];
        for (input, expected) in cases {
            let result = problem2(input, 256);
            assert_eq!(result, expected)
        }
    }
}
