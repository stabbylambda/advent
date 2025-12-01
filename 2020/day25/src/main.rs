use common::math::mod_pow;
use nom::{
    character::complete::{newline, u64},
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
}

type Input = (u64, u64);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_pair(u64, newline, u64).parse(input);

    result.unwrap().1
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    mod_pow(subject, loop_size, 20201227)
}

fn find_loop_size(key: u64) -> u64 {
    for n in 0.. {
        if transform(7, n) == key {
            return n;
        }
    }
    unreachable!()
}

fn problem1(input: &Input) -> u64 {
    let (card_pub, door_pub) = *input;

    let card_loop = find_loop_size(card_pub);
    let door_loop = find_loop_size(door_pub);

    let card_key = transform(door_pub, card_loop);
    let door_key = transform(card_pub, door_loop);

    assert_eq!(card_key, door_key);

    card_key
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, transform};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 14897079)
    }
    #[test]
    fn transform_test() {
        assert_eq!(transform(7, 8), 5764801);
        assert_eq!(transform(7, 11), 17807724);
    }
}
