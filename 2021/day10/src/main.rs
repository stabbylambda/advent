use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let input: Vec<_> = input.lines().collect();

    answer!(problem1(&input));
    answer!(problem2(&input));
}

#[derive(Debug)]
enum Result {
    Valid,
    Incomplete(String),
    Invalid(char),
}

fn problem1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|x| match parse_line(x) {
            Result::Invalid(')') => 3,
            Result::Invalid(']') => 57,
            Result::Invalid('}') => 1197,
            Result::Invalid('>') => 25137,
            _ => 0,
        })
        .sum()
}

fn score_autocomplete(autocomplete: String) -> u128 {
    autocomplete.chars().fold(0, |acc, c| {
        let current = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(),
        };
        acc * 5 + current
    })
}

fn problem2(input: &[&str]) -> u128 {
    let mut scores: Vec<u128> = input
        .iter()
        .filter_map(|x| match parse_line(x) {
            Result::Incomplete(a) => Some(score_autocomplete(a)),
            _ => None,
        })
        .collect();

    scores.sort();
    let mid = scores.len() / 2;
    let mid = scores.get(mid).unwrap();
    *mid
}

fn parse_line(s: &str) -> Result {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c == '{' || c == '(' || c == '[' || c == '<' {
            stack.push(c);
            continue;
        }

        if c == '}' || c == ')' || c == ']' || c == '>' {
            let last_open = stack.pop();
            match (last_open, c) {
                (Some('{'), '}') => continue,
                (Some('['), ']') => continue,
                (Some('('), ')') => continue,
                (Some('<'), '>') => continue,
                _ => return Result::Invalid(c),
            }
        }

        panic!("Invalid char {c}");
    }

    if !stack.is_empty() {
        let autocomplete = stack
            .iter()
            .map(|c| match c {
                '{' => '}',
                '[' => ']',
                '(' => ')',
                '<' => '>',
                _ => panic!(),
            })
            .rev()
            .collect();

        Result::Incomplete(autocomplete)
    } else {
        Result::Valid
    }
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let input: Vec<_> = input.lines().collect();
    let result = problem1(&input);
    assert_eq!(result, 26397)
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let input: Vec<_> = input.lines().collect();
    let result = problem2(&input);
    assert_eq!(result, 288957)
}
