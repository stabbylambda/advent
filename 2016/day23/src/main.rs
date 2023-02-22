use std::{collections::HashMap, fmt::Display};

use common::{
    instructions::{instruction0, instruction1, instruction2, instruction3},
    program::Program,
    registers::{register, value, Register, Value},
};
use nom::{
    branch::alt, character::complete::newline, combinator::map, multi::separated_list1, IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);
    input.print();

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}
type Input = Program<Instruction>;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    Multiply(Register, Value, Value),
    JumpNotZero(Value, Value),
    Toggle(Value),
    Skip,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Copy(a, b) => write!(f, "cpy {} {}", a, b),
            Instruction::Increment(a) => write!(f, "inc {}", a),
            Instruction::Decrement(a) => write!(f, "dec {}", a),
            Instruction::Multiply(a, b, c) => write!(f, "mul {} {} {}", a, b, c),
            Instruction::JumpNotZero(a, b) => write!(f, "jnz {} {}", a, b),
            Instruction::Toggle(a) => write!(f, "tgl {}", a),
            Instruction::Skip => write!(f, "skip"),
        }
    }
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match self {
            // For one-argument instructions, inc becomes dec, and all other one-argument instructions become inc.
            Instruction::Increment(x) => Instruction::Decrement(*x),
            Instruction::Decrement(x) => Instruction::Increment(*x),
            Instruction::Toggle(Value::Register(x)) => Instruction::Increment(*x),

            // For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
            Instruction::JumpNotZero(a, Value::Register(r)) => Instruction::Copy(*a, *r),
            Instruction::Copy(a, b) => Instruction::JumpNotZero(*a, Value::Register(*b)),

            Instruction::Multiply(a, b, c) => Instruction::Multiply(*a, *b, *c),

            // If toggling produces an invalid instruction (like cpy 1 2) and an attempt is later made to execute that instruction, skip it instead.
            Instruction::Skip => Instruction::Skip,
            Instruction::JumpNotZero(_, Value::Literal(_)) => Self::Skip,
            Instruction::Toggle(Value::Literal(_)) => Instruction::Skip,
        };
    }
}

fn parse(input: &str) -> Input {
    let tgl = instruction1("tgl", value, Instruction::Toggle);
    let skip = instruction0("skip", Instruction::Skip);
    let inc = instruction1("inc", register, Instruction::Increment);
    let dec = instruction1("dec", register, Instruction::Decrement);
    let cpy = instruction2("cpy", value, register, Instruction::Copy);
    let jnz = instruction2("jnz", value, value, Instruction::JumpNotZero);
    let mul = instruction3("mul", register, value, value, Instruction::Multiply);

    let result: IResult<&str, Input> = map(
        separated_list1(newline, alt((skip, tgl, inc, dec, cpy, jnz, mul))),
        Input::new,
    )(input);

    result.unwrap().1
}

fn compute(input: &mut Input, registers: &mut HashMap<char, i64>) {
    while let Some(instruction) = input.current() {
        input.counter += match instruction {
            Instruction::Toggle(v) => {
                let v = match *v {
                    Value::Literal(x) => x,
                    Value::Register(r) => *registers.get(&r).unwrap(),
                };

                // only toggle instructions that are in the program
                if let Some(instruction) = input.get_mut(input.counter + v) {
                    instruction.toggle();
                };

                1
            }
            Instruction::Copy(v, r) => {
                let v = match *v {
                    Value::Literal(x) => x,
                    Value::Register(r) => *registers.get(&r).unwrap(),
                };

                registers.entry(*r).and_modify(|x| *x = v);
                1
            }
            Instruction::Increment(r) => {
                registers.entry(*r).and_modify(|x| *x += 1);
                1
            }
            Instruction::Decrement(r) => {
                registers.entry(*r).and_modify(|x| *x -= 1);
                1
            }
            Instruction::Multiply(a, b, d) => {
                let b = match *b {
                    Value::Literal(x) => x,
                    Value::Register(r) => *registers.get(&r).unwrap(),
                };
                let d = match *d {
                    Value::Literal(x) => x,
                    Value::Register(r) => *registers.get(&r).unwrap(),
                };

                registers.entry(*a).and_modify(|a| *a += b * d);
                registers.entry('c').and_modify(|c| *c = 0);
                registers.entry('d').and_modify(|d| *d = 0);

                1
            }
            Instruction::JumpNotZero(v, o) => {
                let v = match *v {
                    Value::Literal(x) => x,
                    Value::Register(r) => *registers.get(&r).unwrap(),
                };

                let o = match *o {
                    Value::Literal(x) => x,
                    Value::Register(r) => *registers.get(&r).unwrap(),
                };

                if v != 0 {
                    o
                } else {
                    1
                }
            }
            Instruction::Skip => 1,
        };
    }
}

fn problem1(input: &Input) -> i64 {
    let mut registers = HashMap::new();
    registers.insert('a', 7);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);

    compute(&mut input.clone(), &mut registers);
    registers[&'a']
}

fn problem2(input: &Input) -> i64 {
    let mut registers = HashMap::new();
    registers.insert('a', 12);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);

    compute(&mut input.clone(), &mut registers);
    registers[&'a']
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 11640)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 479008200)
    }
}
