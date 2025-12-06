use advent_2017_10::{hash, SparseHash};
use nom::{bytes::complete::tag, character::complete::u8, multi::separated_list1, IResult, Parser};

fn main() {
    let input = common::read_input!();

    let answer = problem1(input, 256);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input, 256);
    println!("problem 2 answer: {answer}");
}

fn parse(input: &str) -> Vec<u8> {
    let result: IResult<&str, Vec<u8>> = separated_list1(tag(","), u8).parse(input);

    result.unwrap().1
}

fn problem1(input: &str, max: usize) -> u8 {
    let input = parse(input);
    let rope = hash(input, max, 1);
    rope.check()
}

fn problem2(input: &str, max: usize) -> String {
    let mut input: Vec<u8> = input.bytes().collect();
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
