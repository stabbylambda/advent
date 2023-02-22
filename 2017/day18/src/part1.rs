use std::fmt::Display;

use common::program::{
    parsing::{instruction1, instruction2, register, value},
    registers::{Register, Value},
    Program,
};
use nom::{branch::alt, character::complete::newline, multi::separated_list1, IResult};

type Input = Vec<Instruction>;
#[derive(Debug, Clone, Copy)]
enum Instruction {
    PlaySound(Value),
    Set(Register, Value),
    Add(Register, Value),
    Multiply(Register, Value),
    Mod(Register, Value),
    Recover(Value),
    JumpGreaterThanZero(Value, Value),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::PlaySound(x) => write!(f, "snd {x}"),
            Instruction::Set(x, y) => write!(f, "set {x} {y}"),
            Instruction::Add(x, y) => write!(f, "add {x} {y}"),
            Instruction::Multiply(x, y) => write!(f, "mul {x} {y}"),
            Instruction::Mod(x, y) => write!(f, "mod {x} {y}"),
            Instruction::Recover(x) => write!(f, "rcv {x}"),
            Instruction::JumpGreaterThanZero(x, y) => write!(f, "jgz {x} {y}"),
        }
    }
}

fn parse(input: &str) -> Input {
    let ops = alt((
        instruction1("snd", value, Instruction::PlaySound),
        instruction2("set", register, value, Instruction::Set),
        instruction2("add", register, value, Instruction::Add),
        instruction2("mul", register, value, Instruction::Multiply),
        instruction2("mod", register, value, Instruction::Mod),
        instruction1("rcv", value, Instruction::Recover),
        instruction2("jgz", value, value, Instruction::JumpGreaterThanZero),
    ));

    let result: IResult<&str, Input> = separated_list1(newline, ops)(input);

    result.unwrap().1
}

pub fn problem1(input: &str) -> i64 {
    let instructions = parse(input);
    let mut program = Program::with_data(instructions, None);

    while let Some(instruction) = program.current() {
        program.counter += match *instruction {
            Instruction::PlaySound(x) => {
                let x = program.registers.resolve(x);
                program.data = Some(x);
                1
            }
            Instruction::Set(x, y) => {
                let y = program.registers.resolve(y);
                program.registers.set(x, y);
                1
            }
            Instruction::Add(x, y) => {
                let y = program.registers.resolve(y);
                program.registers.add(x, y);
                1
            }
            Instruction::Multiply(x, y) => {
                let y = program.registers.resolve(y);
                program.registers.entry(x).and_modify(|c| *c *= y);
                1
            }
            Instruction::Mod(x, y) => {
                let y = program.registers.resolve(y);
                program.registers.entry(x).and_modify(|x| {
                    *x %= y;
                });
                1
            }
            Instruction::Recover(x) => {
                let x = program.registers.resolve(x);
                if x != 0 {
                    // the program terminates as soon as we hit a recover
                    return program.data.unwrap();
                }

                1
            }
            Instruction::JumpGreaterThanZero(x, y) => {
                let x = program.registers.resolve(x);
                let y = program.registers.resolve(y);

                match x > 0 {
                    true => y,
                    false => 1,
                }
            }
        };
    }
    unreachable!()
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let result = problem1(input);
    assert_eq!(result, 4)
}
