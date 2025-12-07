use std::collections::HashMap;

use common::{answer, read_input};
use nom::{
    character::complete::{space1, u32},
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    let (total, cycles) = problem(&input);
    answer!(total);
    answer!(cycles);
}

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(space1, u32).parse(input);

    result.unwrap().1
}

fn problem(input: &Input) -> (u32, u32) {
    let mut banks = input.clone();
    let len = banks.len();
    let mut seen: HashMap<Vec<u32>, u32> = HashMap::new();

    for cycle in 0.. {
        // if we've already seen this bank configuration, return the previous cycle we saw it on
        if let Some(previous) = seen.insert(banks.clone(), cycle) {
            return (cycle, cycle - previous);
        }

        // get the max and its position
        let &max = banks.iter().max().unwrap();
        let pos = banks.iter().position(|x| *x == max).unwrap();

        // clear the current position
        banks[pos] = 0;

        // add one to all the other banks in sequence
        for n in 1..=max {
            banks[(pos + (n as usize)) % len] += 1;
        }
    }

    unreachable!("There *must* be a cycle by definition")
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn test() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (total, cycles) = problem(&input);
        assert_eq!(total, 5);
        assert_eq!(cycles, 4);
    }
}
