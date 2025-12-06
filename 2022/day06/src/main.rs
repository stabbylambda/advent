use std::collections::BTreeSet;

fn main() {
    let lines = common::read_input!();

    let answer = problem1(lines);
    println!("problem 1 answer: {answer}");

    let answer = problem2(lines);
    println!("problem 2 answer: {answer}");
}

fn unique_string(count: usize, line: &str) -> u32 {
    let all_chars = line.chars().collect::<Vec<char>>();
    let control = all_chars.windows(count).enumerate().find(|(_, chars)| {
        let set: BTreeSet<&char> = chars.iter().collect();
        set.len() == count
    });

    let (idx, _sequence) = control.unwrap();
    (idx + count) as u32
}

fn problem1(lines: &str) -> u32 {
    unique_string(4, lines)
}

fn problem2(lines: &str) -> u32 {
    unique_string(14, lines)
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let lines = include_str!("../test.txt");
        let result = problem1(lines);
        assert_eq!(result, 7)
    }

    #[test]
    fn second() {
        let lines = include_str!("../test.txt");
        let result = problem2(lines);
        assert_eq!(result, 19)
    }
}
