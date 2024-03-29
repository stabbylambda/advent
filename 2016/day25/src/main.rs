use std::fmt::Display;

use common::program::{
    parsing::{instruction0, instruction1, instruction2, register, value},
    registers::{Register, Registers, Value},
    Program,
};
use nom::{
    branch::alt,
    character::complete::newline,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
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
    Transmit(Value),
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
            Instruction::Transmit(a) => write!(f, "out {}", a),
            Instruction::Skip => write!(f, "skip"),
        }
    }
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match *self {
            // For one-argument instructions, inc becomes dec, and all other one-argument instructions become inc.
            Instruction::Increment(x) => Instruction::Decrement(x),
            Instruction::Decrement(x) => Instruction::Increment(x),
            Instruction::Toggle(Value::Register(x)) => Instruction::Increment(x),

            // For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
            Instruction::JumpNotZero(a, Value::Register(r)) => Instruction::Copy(a, r),
            Instruction::Copy(a, b) => Instruction::JumpNotZero(a, Value::Register(b)),

            Instruction::Multiply(a, b, c) => Instruction::Multiply(a, b, c),

            Instruction::Transmit(x) => Instruction::Transmit(x),

            // If toggling produces an invalid instruction (like cpy 1 2) and an attempt is later made to execute that instruction, skip it instead.
            Instruction::Skip => Instruction::Skip,
            Instruction::JumpNotZero(_, Value::Literal(_)) => Self::Skip,
            Instruction::Toggle(Value::Literal(_)) => Instruction::Skip,
        };
    }
}

fn parse(input: &str) -> Input {
    let out = |s| instruction1("out", value, Instruction::Transmit)(s);
    let tgl = |s| instruction1("tgl", value, Instruction::Toggle)(s);
    let skip = |s| instruction0("skip", Instruction::Skip)(s);
    let inc = |s| instruction1("inc", register, Instruction::Increment)(s);
    let dec = |s| instruction1("dec", register, Instruction::Decrement)(s);
    let cpy = |s| instruction2("cpy", value, register, Instruction::Copy)(s);
    let jnz = |s| instruction2("jnz", value, value, Instruction::JumpNotZero)(s);

    // this parser is able to recognize the pattern that is multiplication, if it fails, we'll just get the deoptimized
    // code
    let optimized_mul = |s| {
        map_res(
            tuple((
                terminated(cpy, newline),
                terminated(inc, newline),
                terminated(dec, newline),
                terminated(jnz, newline),
                terminated(dec, newline),
                jnz,
            )),
            |(x0, x1, x2, x3, x4, x5)| {
                let Instruction::Copy(op1, temp1_1) = x0 else {panic!()};
                let Instruction::Increment(store) = x1 else {panic!()};
                let Instruction::Decrement(temp1_2) = x2 else {panic!()};
                let Instruction::JumpNotZero(Value::Register(temp1_3), _dec) = x3 else {panic!()};
                let Instruction::Decrement(op2_1) = x4 else {panic!()};
                let Instruction::JumpNotZero(Value::Register(op2_2), _dec) = x5 else {panic!()};

                if temp1_1 == temp1_2 && temp1_2 == temp1_3 && op2_1 == op2_2 {
                    Ok(Instruction::Multiply(store, op1, Value::Register(op2_1)))
                } else {
                    Err(nom::Err::Error("Multiply not optimizable"))
                }
            },
        )(s)
    };

    let ops = alt((skip, optimized_mul, tgl, inc, dec, cpy, jnz, out));
    let result: IResult<&str, Input> = map(separated_list1(newline, ops), Input::new)(input);

    result.unwrap().1
}

fn compute(input: &mut Input, registers: &mut Registers) -> Vec<u32> {
    let mut transmission: Vec<_> = vec![];
    while let Some(instruction) = input.current() {
        input.counter += match *instruction {
            Instruction::Transmit(v) => {
                let v = registers.resolve(v);
                transmission.push(v as u32);

                if transmission.len() == 10 {
                    return transmission;
                }

                1
            }
            Instruction::Toggle(v) => {
                let v = registers.resolve(v);

                // only toggle instructions that are in the program
                if let Some(instruction) = input.get_mut(input.counter + v) {
                    instruction.toggle();
                };

                1
            }
            Instruction::Copy(v, r) => {
                let v = registers.resolve(v);
                registers.set(r, v);
                1
            }
            Instruction::Increment(r) => {
                registers.add(r, 1);
                1
            }
            Instruction::Decrement(r) => {
                registers.add(r, -1);
                1
            }
            Instruction::Multiply(a, b, d) => {
                let bv = registers.resolve(b);
                let dv = registers.resolve(d);

                registers.add(a, bv * dv);
                if let Value::Register(r) = b {
                    registers.set(r, 0);
                };
                if let Value::Register(r) = d {
                    registers.set(r, 0);
                };

                1
            }
            Instruction::JumpNotZero(v, o) => {
                let v = registers.resolve(v);
                let o = registers.resolve(o);

                if v != 0 {
                    o
                } else {
                    1
                }
            }
            Instruction::Skip => 1,
        };
    }

    transmission
}

fn problem1(input: &Input) -> i64 {
    for a in 0.. {
        let mut registers = Registers::new(a);
        let transmission = compute(&mut input.clone(), &mut registers);
        if transmission == vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1] {
            return a;
        }
    }

    0
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 198)
    }
}
