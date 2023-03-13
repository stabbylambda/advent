use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let answer = problem1(input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input);
    println!("problem 2 answer: {answer}");
}

fn exactly(input: &str, count: usize) -> bool {
    input
        .chars()
        .any(|c| input.chars().filter(|x| *x == c).count() == count)
}

fn problem1(input: &str) -> usize {
    let exactly_two = input.lines().filter(|s| exactly(s, 2)).count();
    let exactly_three = input.lines().filter(|s| exactly(s, 3)).count();

    exactly_two * exactly_three
}

fn problem2(input: &str) -> String {
    for x in input.lines() {
        for y in input.lines() {
            let xc: HashSet<(usize, char)> = x.chars().enumerate().collect();
            let yc: HashSet<(usize, char)> = y.chars().enumerate().collect();
            let diff: Vec<_> = xc.difference(&yc).collect();

            if diff.len() == 1 {
                let index_to_remove = diff[0].0;
                let (a, b) = x.split_at(index_to_remove);
                let s = format!("{}{}", a, &b[1..]);

                return s;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let result = problem1(input);
        assert_eq!(result, 12)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let result = problem2(input);
        assert_eq!(result, "fgij")
    }
}
