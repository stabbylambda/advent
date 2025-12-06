use std::fmt::Display;

use common::program::{
    parsing::{instruction2, register, value},
    registers::{Register, Value},
    Program,
};
use nom::{branch::alt, character::complete::newline, multi::separated_list1, IResult, Parser};

fn main() {
    let input = common::read_input!();

    let answer = problem1(input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Instruction>;
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Set(Register, Value),
    Subtract(Register, Value),
    Multiply(Register, Value),
    JumpNotZero(Value, Value),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Set(x, y) => write!(f, "set {x} {y}"),
            Instruction::Subtract(x, y) => write!(f, "sub {x} {y}"),
            Instruction::Multiply(x, y) => write!(f, "mul {x} {y}"),
            Instruction::JumpNotZero(x, y) => write!(f, "jnz {x} {y}"),
        }
    }
}

fn parse(input: &str) -> Input {
    let ops = alt((
        instruction2("set", register, value, Instruction::Set),
        instruction2("sub", register, value, Instruction::Subtract),
        instruction2("mul", register, value, Instruction::Multiply),
        instruction2("jnz", value, value, Instruction::JumpNotZero),
    ));

    let result: IResult<&str, Input> = separated_list1(newline, ops).parse(input);

    result.unwrap().1
}

pub fn problem1(input: &str) -> i64 {
    let instructions = parse(input);
    let mut program = Program::<Instruction, ()>::new(instructions);
    let mut mul_invoked = 0;

    while let Some(instruction) = program.current() {
        program.counter += match *instruction {
            Instruction::Set(x, y) => {
                let y = program.registers.resolve(y);
                program.registers.set(x, y);
                1
            }
            Instruction::Subtract(x, y) => {
                let y = program.registers.resolve(y);
                program.registers.add(x, -y);
                1
            }
            Instruction::Multiply(x, y) => {
                mul_invoked += 1;
                let y = program.registers.resolve(y);
                program.registers.entry(x).and_modify(|c| *c *= y);
                1
            }
            Instruction::JumpNotZero(x, y) => {
                let x = program.registers.resolve(x);
                let y = program.registers.resolve(y);

                match x != 0 {
                    true => y,
                    false => 1,
                }
            }
        };
    }
    mul_invoked
}

pub fn problem2(input: &str) -> i64 {
    let instructions = parse(input);
    let Instruction::Set(_, Value::Literal(b)) = instructions[0] else {
        panic!("The first instruction wasn't set!");
    };

    let b = 100_000 + b * 100;
    let c = b + 17_000;

    println!("Finding composite numbers in [{b}, {c}]");

    let mut composites = 0;
    for n in (b..=c).step_by(17) {
        let mut d = 2;
        while n % d != 0 {
            d += 1;
        }

        if n != d {
            composites += 1;
        }
    }

    composites
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let result = problem1(input);
        assert_eq!(result, 6241)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let result = problem2(input);
        assert_eq!(result, 909)
    }
}
