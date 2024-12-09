use std::time::Instant;

use nom::IResult;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = todo!();

    result.unwrap().1
}

fn problem1(_input: &Input) -> u32 {
    todo!()
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
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
