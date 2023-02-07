use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::complete::{alphanumeric1, anychar, char},
    combinator::{opt, value},
    multi::count,
    sequence::{delimited, preceded},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = Vec<&'a str>;

fn parse(input: &str) -> Input {
    input.lines().collect()
}

fn escape(s: &str) -> IResult<&str, Option<String>> {
    delimited(
        char('"'),
        opt(escaped_transform(
            alphanumeric1,
            '\\',
            alt((
                value("-", tag("\"")),
                value("-", tag("\\")),
                value("-", preceded(tag("x"), count(anychar, 2))),
            )),
        )),
        char('"'),
    )(s)
}

fn problem1(input: &Input) -> usize {
    let (originals, parsed): (Vec<usize>, Vec<usize>) = input
        .iter()
        .map(|x| {
            let original = x.len();
            let parsed = escape(x).unwrap().1;
            let x = parsed.map(|y| y.len()).unwrap_or(0);
            (original, x)
        })
        .unzip();

    let originals: usize = originals.iter().sum();
    let parsed: usize = parsed.iter().sum();

    originals - parsed
}

fn problem2(input: &Input) -> usize {
    let (originals, encoded): (Vec<usize>, Vec<usize>) = input
        .iter()
        .map(|x| {
            let original = x.len();
            let encoded = x.replace('\\', "\\\\").replace('"', "\\\"").len() + 2;

            (original, encoded)
        })
        .unzip();

    let originals: usize = originals.iter().sum();
    let encoded: usize = encoded.iter().sum();

    encoded - originals
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 12)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 19)
    }
}
