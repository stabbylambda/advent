use advent_2018_16::{Instruction, Opcode};
use common::nom::usize;
use common::program::parsing::instruction3;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
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

type Input = (usize, Vec<Instruction<Opcode>>);

fn parse(input: &str) -> Input {
    let opcode_instruction = |name: &'static str, opcode: Opcode| {
        instruction3(name, usize, usize, usize, Instruction::new(opcode))
    };
    let result: IResult<&str, Input> = separated_pair(
        preceded(tag("#ip "), usize),
        newline,
        separated_list1(
            newline,
            alt((
                opcode_instruction("addr", Opcode::Addr),
                opcode_instruction("addi", Opcode::Addi),
                opcode_instruction("mulr", Opcode::Mulr),
                opcode_instruction("muli", Opcode::Muli),
                opcode_instruction("banr", Opcode::Banr),
                opcode_instruction("bani", Opcode::Bani),
                opcode_instruction("borr", Opcode::Borr),
                opcode_instruction("bori", Opcode::Bori),
                opcode_instruction("setr", Opcode::Setr),
                opcode_instruction("seti", Opcode::Seti),
                opcode_instruction("gtir", Opcode::Gtir),
                opcode_instruction("gtri", Opcode::Gtri),
                opcode_instruction("gtrr", Opcode::Gtrr),
                opcode_instruction("eqir", Opcode::Eqir),
                opcode_instruction("eqri", Opcode::Eqri),
                opcode_instruction("eqrr", Opcode::Eqrr),
            )),
        ),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let registers = execute(input, 0, false);
    registers[0]
}

fn execute((bound, program): &Input, starting: usize, terminate_early: bool) -> Vec<usize> {
    let mut registers = vec![starting, 0, 0, 0, 0, 0];
    let mut ip = 0;

    while let Some(instruction) = program.get(ip) {
        // break as soon as the first register switches to zero, we've got our number
        if terminate_early && registers[0] != starting {
            break;
        }
        registers[*bound] = ip;
        registers = instruction.execute(&registers);
        ip = registers[*bound] + 1;
    }

    registers
}

fn problem2(input: &Input) -> usize {
    let registers = execute(input, 1, true);
    // our particular input happens to be in the fourth register
    let x = registers[3];
    // find the sum of all the divisors of the number (this is what the assembly is doing)
    (1..=x).filter_map(|n| (x % n == 0).then_some(n)).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 6)
    }
}
