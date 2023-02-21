use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{anychar, char, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let programs = "abcdefghijklmnop";

    let answer = problem1(programs, &input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(programs, &input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Instruction>;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Spin(u32),
    Exchange(u32, u32),
    Partner(char, char),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        char(','),
        alt((
            map(preceded(char('s'), u32), Instruction::Spin),
            map(
                preceded(char('x'), separated_pair(u32, char('/'), u32)),
                |(x, y)| Instruction::Exchange(x, y),
            ),
            map(
                preceded(char('p'), separated_pair(anychar, char('/'), anychar)),
                |(x, y)| Instruction::Partner(x, y),
            ),
        )),
    )(input);

    result.unwrap().1
}

fn problem1(programs: &str, input: &Input) -> String {
    let mut v: Vec<char> = programs.chars().into_iter().collect();
    for i in input {
        match i {
            Instruction::Spin(num) => v.rotate_right(*num as usize),
            Instruction::Exchange(a, b) => v.swap(*a as usize, *b as usize),
            Instruction::Partner(a, b) => {
                let a = v.iter().position(|x| x == a).unwrap();
                let b = v.iter().position(|x| x == b).unwrap();

                v.swap(a, b)
            }
        }
    }
    v.iter().collect()
}

fn problem2(programs: &str, input: &Input) -> String {
    let times = 1_000_000_000;
    let mut current = 0;
    let mut map: HashMap<String, u32> = HashMap::new();

    let mut s = programs.to_string();
    while current < times {
        s = problem1(&s, input);
        // println!("{current:10} {s}");

        // if we've seen this before, we have a cycle
        if let Some(old_index) = map.insert(s.clone(), current) {
            // skip cycle_count times until we get up close to a billion
            let cycle_size = old_index.abs_diff(current);
            let cycle_count = (times - current) / cycle_size;
            let new_index = current + (cycle_size * cycle_count);

            current = new_index;

            // clear the map so we don't repeatedly find cycles
            map.clear();
        }
        current += 1;
    }

    s
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1("abcde", &input);
        assert_eq!(result, "baedc")
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2("abcde", &input);
        assert_eq!(result, "baedc")
    }
}
