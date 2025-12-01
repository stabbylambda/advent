use std::{collections::BTreeMap, fmt::Display};

use common::program::{
    parsing::{instruction1, instruction2, register, value},
    registers::{Register, Value},
};
use nom::{branch::alt, character::complete::newline, multi::separated_list1, IResult, Parser};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Instruction>;
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Input(Register),
    Add(Register, Value),
    Multiply(Register, Value),
    Divide(Register, Value),
    Modulo(Register, Value),
    Equal(Register, Value),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Input(x) => write!(f, "inp {x}"),
            Instruction::Add(a, b) => write!(f, "add {a} {b}"),
            Instruction::Multiply(a, b) => write!(f, "mul {a} {b}"),
            Instruction::Divide(a, b) => write!(f, "div {a} {b}"),
            Instruction::Modulo(a, b) => write!(f, "mod {a} {b}"),
            Instruction::Equal(a, b) => write!(f, "eql {a} {b}"),
        }
    }
}

fn parse(input: &str) -> Input {
    let ops = alt((
        instruction1("inp", register, Instruction::Input),
        instruction2("mul", register, value, Instruction::Multiply),
        instruction2("add", register, value, Instruction::Add),
        instruction2("div", register, value, Instruction::Divide),
        instruction2("mod", register, value, Instruction::Modulo),
        instruction2("eql", register, value, Instruction::Equal),
    ));

    let result: IResult<&str, Input> = separated_list1(newline, ops).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Registers([i64; 4]);

impl Registers {
    pub fn to_index(c: char) -> usize {
        match c {
            'w' => 0,
            'x' => 1,
            'y' => 2,
            'z' => 3,
            _ => unreachable!(),
        }
    }
    pub fn resolve(&self, v: Value) -> i64 {
        match v {
            Value::Literal(x) => x,
            Value::Register(c) => self.0[Self::to_index(c)],
        }
    }

    pub fn set(&mut self, c: char, i: i64) {
        self.0[Self::to_index(c)] = i;
    }

    pub fn get(&mut self, c: char) -> &mut i64 {
        &mut self.0[Self::to_index(c)]
    }
}
struct ImmutableProgram<'a> {
    program: &'a [Instruction],
    counter: usize,
    registers: Registers,
}

impl<'a> ImmutableProgram<'a> {
    fn new(program: &'a [Instruction]) -> Self {
        Self {
            program,
            counter: 0,
            registers: Registers([0, 0, 0, 0]),
        }
    }

    fn to_memo_key(&self) -> (Registers, usize) {
        (self.registers, self.counter)
    }
}

impl<'a> Clone for ImmutableProgram<'a> {
    fn clone(&self) -> Self {
        Self {
            program: self.program,
            counter: self.counter,
            registers: self.registers,
        }
    }
}

fn execute(
    digits: &[i64],
    program: ImmutableProgram,
    memo: &mut BTreeMap<(Registers, usize), Option<i64>>,
) -> Option<i64> {
    if let Some(result) = memo.get(&program.to_memo_key()) {
        return *result;
    }

    'inputs: for &input in digits {
        let mut fork = false;
        let mut program = program.clone();

        while let Some(instruction) = program.program.get(program.counter) {
            match *instruction {
                Instruction::Input(_) if fork => {
                    if let Some(best) = execute(digits, program.clone(), memo) {
                        let place = 10i64.pow(best.ilog10() + 1);
                        let result = Some(input * place + best);
                        memo.insert(program.to_memo_key(), result);
                        return result;
                    } else {
                        continue 'inputs;
                    }
                }
                Instruction::Input(a) => {
                    // once we read the first input, we need to make sure to fork on the next one
                    fork = true;
                    program.registers.set(a, input);
                }
                Instruction::Add(a, b) => {
                    let b = program.registers.resolve(b);
                    *program.registers.get(a) += b;
                }
                Instruction::Multiply(a, b) => {
                    let b = program.registers.resolve(b);
                    *program.registers.get(a) *= b;
                }
                Instruction::Divide(a, b) => {
                    let b = program.registers.resolve(b);
                    *program.registers.get(a) /= b;
                }
                Instruction::Modulo(a, b) => {
                    let b = program.registers.resolve(b);
                    *program.registers.get(a) %= b;
                }
                Instruction::Equal(a, b) => {
                    let b_val = program.registers.resolve(b);
                    let a = program.registers.get(a);
                    let a_val = *a;

                    *a = (a_val == b_val) as i64;
                }
            }

            program.counter += 1;
        }

        let z = *program.registers.get('z');
        if z == 0 {
            memo.insert(program.to_memo_key(), Some(input));
            return Some(input);
        }
    }

    memo.insert(program.to_memo_key(), None);
    None
}

fn problem1(input: &Input) -> i64 {
    let program = ImmutableProgram::new(input);
    let digits: Vec<i64> = (1..=9).rev().collect();

    execute(&digits, program, &mut BTreeMap::new()).unwrap()
}

fn problem2(input: &Input) -> i64 {
    let program = ImmutableProgram::new(input);
    let digits: Vec<i64> = (1..=9).collect();

    execute(&digits, program, &mut BTreeMap::new()).unwrap()
}
