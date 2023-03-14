use std::collections::{HashMap, HashSet};

use advent_2018_16::{Instruction, Opcode};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = (Vec<InstructionSample>, Vec<Instruction<usize>>);

#[derive(Debug)]
struct InstructionSample {
    before: Vec<usize>,
    instruction: Instruction<usize>,
    after: Vec<usize>,
}

impl InstructionSample {
    fn matching(&self) -> HashMap<Opcode, bool> {
        [
            (Opcode::Addr, self.instruction.addr(&self.before[..])),
            (Opcode::Addi, self.instruction.addi(&self.before[..])),
            (Opcode::Mulr, self.instruction.mulr(&self.before[..])),
            (Opcode::Muli, self.instruction.muli(&self.before[..])),
            (Opcode::Banr, self.instruction.banr(&self.before[..])),
            (Opcode::Bani, self.instruction.bani(&self.before[..])),
            (Opcode::Borr, self.instruction.borr(&self.before[..])),
            (Opcode::Bori, self.instruction.bori(&self.before[..])),
            (Opcode::Setr, self.instruction.setr(&self.before[..])),
            (Opcode::Seti, self.instruction.seti(&self.before[..])),
            (Opcode::Gtir, self.instruction.gtir(&self.before[..])),
            (Opcode::Gtri, self.instruction.gtri(&self.before[..])),
            (Opcode::Gtrr, self.instruction.gtrr(&self.before[..])),
            (Opcode::Eqir, self.instruction.eqir(&self.before[..])),
            (Opcode::Eqri, self.instruction.eqri(&self.before[..])),
            (Opcode::Eqrr, self.instruction.eqrr(&self.before[..])),
        ]
        .into_iter()
        .map(|(k, v)| (k, v == self.after))
        .collect()
    }
    fn test(&self) -> usize {
        self.matching().values().filter(|x| **x).count()
    }
}

fn parse(input: &str) -> Input {
    let instruction = |s| {
        map(separated_list1(tag(" "), u32), |x| Instruction {
            opcode: x[0] as usize,
            input_a: x[1] as usize,
            input_b: x[2] as usize,
            output: x[3] as usize,
        })(s)
    };

    let result: IResult<&str, Input> = separated_pair(
        separated_list1(
            tag("\n"),
            map(
                tuple((
                    delimited(
                        tag("Before: ["),
                        separated_list1(tag(", "), map(u32, |x| x as usize)),
                        tag("]\n"),
                    ),
                    terminated(instruction, tag("\n")),
                    (delimited(
                        tag("After:  ["),
                        separated_list1(tag(", "), map(u32, |x| x as usize)),
                        tag("]\n"),
                    )),
                )),
                |(before, instruction, after)| InstructionSample {
                    before,
                    instruction,
                    after,
                },
            ),
        ),
        tag("\n\n\n"),
        separated_list1(newline, instruction),
    )(input);

    result.unwrap().1
}

fn problem1((samples, _program): &Input) -> usize {
    samples.iter().map(|x| x.test()).filter(|x| *x >= 3).count()
}

fn get_mappings(samples: &[InstructionSample]) -> HashMap<usize, Opcode> {
    // go through and find all the samples that pass
    let mut opcode_map: HashMap<usize, HashSet<Opcode>> = HashMap::new();

    for x in samples {
        let matching_opcodes: HashSet<Opcode> = x
            .matching()
            .into_iter()
            .filter_map(|(k, v)| v.then_some(k))
            .collect();

        opcode_map
            .entry(x.instruction.opcode)
            .and_modify(|set| {
                set.extend(matching_opcodes.clone());
            })
            .or_insert(matching_opcodes);
    }

    let mut final_map: HashMap<usize, Opcode> = HashMap::new();

    // reduce everything down to a mapping of one
    while final_map.len() != 16 {
        // find the ones that are 1
        for (key, opcodes) in &opcode_map {
            if opcodes.len() == 1 {
                let opcode = *opcodes.iter().next().unwrap();
                final_map.insert(*key, opcode);
            }
        }

        // remove all the solo values from the other potential maps
        for opcode in final_map.values() {
            for v in opcode_map.values_mut() {
                v.remove(opcode);
            }
        }
    }

    final_map
}

fn problem2((samples, program): &Input) -> usize {
    let final_map = get_mappings(samples);
    let mut registers = vec![0, 0, 0, 0];
    for instruction in program {
        registers = instruction.execute(&final_map, &registers);
    }

    registers[0]
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1)
    }

    #[test]
    fn second() {
        let input = include_str!("../input.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 649)
    }
}
