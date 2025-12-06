use common::nom::single_digit;
use nom::{multi::many1, IResult, Parser};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(single_digit).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    let mut v = input.clone();
    v.push(*v.first().unwrap());

    v.windows(2)
        .map(|w| if w[0] == w[1] { w[0] } else { 0 })
        .sum()
}

fn problem2(input: &Input) -> u32 {
    let (first, second) = input.split_at(input.len() / 2);
    first
        .iter()
        .zip(second.iter())
        .map(|(&a, &b)| if a == b { 2 * a } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let cases = [("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem1(&input);
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn second() {
        let cases = [
            ("1212", 6),
            ("1221", 0),
            ("123425", 4),
            ("123123", 12),
            ("12131415", 4),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem2(&input);
            assert_eq!(result, expected)
        }
    }
}
