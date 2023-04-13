use std::{collections::BTreeMap, fmt::Debug, ops::RangeFrom};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    AsChar, IResult, InputIter, Slice,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let input = include_str!("../input2.txt");
    let input = input
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");
    let input = parse(&input);

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = (BTreeMap<u32, Rule>, Vec<String>);

#[derive(Debug, PartialEq, Eq)]
enum Rule {
    Literal(char),
    String(Vec<u32>),
    Or(Vec<u32>, Vec<u32>),
}

fn parse(input: &str) -> Input {
    fn rule(s: &str) -> IResult<&str, Rule> {
        alt((
            delimited(tag("\""), map(anychar, Rule::Literal), tag("\"")),
            map(
                separated_pair(
                    separated_list1(tag(" "), u32),
                    tag(" | "),
                    separated_list1(tag(" "), u32),
                ),
                |(a, b)| Rule::Or(a, b),
            ),
            map(separated_list1(tag(" "), u32), Rule::String),
        ))(s)
    }

    let rules = |s| {
        map(
            separated_list1(newline, separated_pair(u32, tag(": "), rule)),
            |x| x.into_iter().collect(),
        )(s)
    };
    let messages = |s| separated_list1(newline, map(take_until("\n"), |s: &str| s.to_string()))(s);
    let result: IResult<&str, Input> = separated_pair(rules, tag("\n\n"), messages)(input);

    result.unwrap().1
}

fn build_string_parser<'a, I>(
    keys: &'a [u32],
    rules: &'a BTreeMap<u32, Rule>,
) -> impl Fn(I) -> IResult<I, Vec<char>, nom::error::Error<I>> + 'a
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + Copy + Debug,
    <I as InputIter>::Item: AsChar,
{
    move |input: I| {
        let mut input = input;
        let mut result = vec![];
        for k in keys {
            let (p_i, p_r) = build_parser(*k, rules)(input)?;
            result.push(p_r);
            input = p_i;
        }

        Ok((input, result.into_iter().flatten().collect()))
    }
}

// clippy is wrong
#[allow(clippy::needless_lifetimes)]
fn build_parser<'a, I>(
    key: u32,
    rules: &'a BTreeMap<u32, Rule>,
) -> impl Fn(I) -> IResult<I, Vec<char>, nom::error::Error<I>> + 'a
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + Copy + Debug,
    <I as InputIter>::Item: AsChar,
{
    move |s: I| {
        let rule = rules.get(&key).unwrap();
        match rule {
            Rule::Literal(x) => map(char(*x), |c| vec![c])(s),
            Rule::String(v) => build_string_parser(v, rules)(s),
            Rule::Or(a, b) => {
                let a = build_string_parser(a, rules);
                let b = build_string_parser(b, rules);
                alt((a, b))(s)
            }
        }
    }
}

fn problem1((rules, messages): &Input) -> usize {
    let parser = |s| build_parser::<&str>(0, rules)(s);

    messages
        .iter()
        .filter(|x| match parser(x) {
            // did we consume the entire input?
            Result::Ok((rest, _result)) => rest.is_empty(),
            Err(_) => false,
        })
        .count()
}

fn problem2((rules, messages): &Input) -> usize {
    let parser = |s| build_parser::<&str>(0, rules)(s);

    messages
        .iter()
        .filter(|x| match parser(x) {
            // did we consume the entire input?
            Result::Ok((rest, _result)) => rest.is_empty(),
            Err(_err) => false,
        })
        .count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    #[ignore]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = input
            .replace("8: 42", "8: 42 | 42 8")
            .replace("11: 42 31", "11: 42 31 | 42 11 31");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 12)
    }
}

//         if let &[a, b, c] = keys {
//             let (input, result_a) = build_parser(a, rules)(input)?;
//             let (input, result_b) = build_parser(b, rules)(input)?;
//             let (input, result_c) = build_parser(c, rules)(input)?;

//             let result = vec![result_a, result_b, result_c]
//                 .into_iter()
//                 .flatten()
//                 .collect();

//             Ok((input, result))
//         } else if let &[a, b] = keys {
//             let (input, result_a) = build_parser(a, rules)(input)?;
//             let (input, result_b) = build_parser(b, rules)(input)?;

//             let result = vec![result_a, result_b].into_iter().flatten().collect();

//             Ok((input, result))
//         } else if let &[a] = keys {
//             let (input, result_a) = build_parser(a, rules)(input)?;

//             let result = vec![result_a].into_iter().flatten().collect();

//             Ok((input, result))
//         } else {
//             panic!()
//         }
