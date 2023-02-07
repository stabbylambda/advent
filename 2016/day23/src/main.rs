use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);
    input.print();

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}
type Input = Program;
#[derive(Clone)]
struct Program {
    instructions: Vec<Instruction>,
    counter: i32,
}
impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions,
            counter: 0,
        }
    }

    fn print(&self) {
        println!();
        for (i, x) in self.instructions.iter().enumerate() {
            let pointer = if i == self.counter as usize { ">" } else { " " };
            println!("{}{x}", pointer)
        }
        println!();
    }

    fn get(&self, idx: i32) -> Option<&Instruction> {
        self.instructions.get(idx as usize)
    }

    fn get_mut(&mut self, idx: i32) -> Option<&mut Instruction> {
        self.instructions.get_mut(idx as usize)
    }

    fn current(&self) -> Option<&Instruction> {
        self.get(self.counter)
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Literal(i32),
    Register(Register),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(x) => write!(f, "{x}"),
            Value::Register(x) => write!(f, "{x}"),
        }
    }
}

type Register = char;

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
    let value = |s| alt((map(i32, Value::Literal), map(anychar, Value::Register)))(s);

    let tgl = |s| map(preceded(tag("tgl "), value), Instruction::Toggle)(s);
    let skip = |s| map(tag("skip"), |_| Instruction::Skip)(s);
    let inc = |s| map(preceded(tag("inc "), anychar), Instruction::Increment)(s);
    let dec = |s| map(preceded(tag("dec "), anychar), Instruction::Decrement)(s);
    let cpy = |s| {
        map(
            preceded(tag("cpy "), separated_pair(value, tag(" "), anychar)),
            |(v, r)| Instruction::Copy(v, r),
        )(s)
    };
    let jnz = |s| {
        map(
            preceded(tag("jnz "), separated_pair(value, tag(" "), value)),
            |(r, o)| Instruction::JumpNotZero(r, o),
        )(s)
    };
    let mul = |s| {
        map(
            tuple((
                tag("mul "),
                terminated(anychar, tag(" ")),
                terminated(value, tag(" ")),
                value,
            )),
            |(_, a, b, c)| Instruction::Multiply(a, b, c),
        )(s)
    };

    let result: IResult<&str, Input> = map(
        separated_list1(newline, alt((skip, tgl, inc, dec, cpy, jnz, mul))),
        Program::new,
    )(input);

    result.unwrap().1
}

fn compute(input: &mut Input, registers: &mut HashMap<char, i32>) {
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

fn problem1(input: &Input) -> i32 {
    let mut registers = HashMap::new();
    registers.insert('a', 7);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);

    compute(&mut input.clone(), &mut registers);
    registers[&'a']
}

fn problem2(input: &Input) -> i32 {
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
