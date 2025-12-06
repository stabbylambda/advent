use std::collections::BTreeSet;

use advent_2018_19::{ElfCode, Opcode};

fn main() {
    let input = common::read_input!();
    let input = ElfCode::parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input = ElfCode;

fn problem(elfcode: &Input) -> (usize, usize) {
    let (mut answer1, mut answer2) = (0, 0);
    let mut registers = vec![0; 6];
    let mut ip = 0;
    let mut seen = BTreeSet::new();

    while let Some(instruction) = elfcode.program.get(ip) {
        registers[elfcode.bound] = ip;
        registers = instruction.execute(&registers);
        ip = registers[elfcode.bound] + 1;

        // only the eqrr line matters, so we need to look at the values coming out of that register
        if instruction.opcode == Opcode::Eqrr {
            let x = registers[instruction.input_a];
            // this is the first time we've seen it
            if seen.is_empty() {
                answer1 = x;
            }

            // once we hit a cycle, it's the value before the cycle
            if !seen.insert(x) {
                break;
            } else {
                answer2 = x;
            }
        }
    }

    (answer1, answer2)
}

#[cfg(test)]
mod test {
    use advent_2018_19::ElfCode;

    use crate::problem;
    #[test]
    #[ignore = "way too slow, takes 1m11s in release mode on my machine"]
    fn first() {
        let input = common::read_input!();
        let input = ElfCode::parse(input);
        let (answer1, _answer2) = problem(&input);
        assert_eq!(answer1, 3941014);
    }
}
