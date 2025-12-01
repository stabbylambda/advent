use std::{collections::VecDeque, fmt::Display};

use common::program::{
    parsing::{instruction1, instruction2, register, value},
    registers::{Register, Value},
    Program,
};
use nom::{branch::alt, character::complete::newline, multi::separated_list1, IResult, Parser};

type Input = Vec<Instruction>;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Send(Value),
    Set(Register, Value),
    Add(Register, Value),
    Multiply(Register, Value),
    Mod(Register, Value),
    Receive(Register),
    JumpGreaterThanZero(Value, Value),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Send(x) => write!(f, "snd {x}"),
            Instruction::Set(x, y) => write!(f, "set {x} {y}"),
            Instruction::Add(x, y) => write!(f, "add {x} {y}"),
            Instruction::Multiply(x, y) => write!(f, "mul {x} {y}"),
            Instruction::Mod(x, y) => write!(f, "mod {x} {y}"),
            Instruction::Receive(x) => write!(f, "rcv {x}"),
            Instruction::JumpGreaterThanZero(x, y) => write!(f, "jgz {x} {y}"),
        }
    }
}

fn parse(input: &str) -> Input {
    let ops = alt((
        instruction1("snd", value, Instruction::Send),
        instruction2("set", register, value, Instruction::Set),
        instruction2("add", register, value, Instruction::Add),
        instruction2("mul", register, value, Instruction::Multiply),
        instruction2("mod", register, value, Instruction::Mod),
        instruction1("rcv", register, Instruction::Receive),
        instruction2("jgz", value, value, Instruction::JumpGreaterThanZero),
    ));

    let result: IResult<&str, Input> = separated_list1(newline, ops).parse(input);

    result.unwrap().1
}

fn compute(program: &mut Program2, other_queue: &mut Queue) -> bool {
    let mut instructions_executed = false;
    while let Some(instruction) = program.current() {
        program.counter += match *instruction {
            Instruction::Send(x) => {
                let x = program.registers.resolve(x);
                program.data.send(x);
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
                program.registers.mul(x, y);
                1
            }
            Instruction::Mod(x, y) => {
                let y = program.registers.resolve(y);
                program.registers.entry(x).and_modify(|x| {
                    *x %= y;
                });
                1
            }
            Instruction::Receive(x) => {
                if let Some(received) = other_queue.receive() {
                    program.registers.set(x, received);
                    1
                } else {
                    // we tried to receive from the other queue, but it's empty
                    break;
                }
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
        // we've executed at least one instruction
        instructions_executed = true;
    }
    instructions_executed
}

type Program2 = Program<Instruction, Queue>;

#[derive(Debug)]
struct Queue {
    items: VecDeque<i64>,
    total_sent: u32,
}

impl Queue {
    fn new() -> Self {
        Queue {
            items: VecDeque::new(),
            total_sent: 0,
        }
    }

    fn send(&mut self, value: i64) {
        self.items.push_back(value);
        self.total_sent += 1;
    }

    fn receive(&mut self) -> Option<i64> {
        self.items.pop_front()
    }
}

pub fn problem2(input: &str) -> u32 {
    let instructions = parse(input);
    let mut pid0: Program2 = Program::with_data(instructions.clone(), Queue::new());
    pid0.registers.set('p', 0);

    let mut pid1: Program2 = Program::with_data(instructions, Queue::new());
    pid1.registers.set('p', 1);

    let mut deadlocked = false;
    while !deadlocked {
        let executed0 = compute(&mut pid0, &mut pid1.data);
        let executed1 = compute(&mut pid1, &mut pid0.data);

        deadlocked = !executed0 && !executed1;
    }

    pid1.data.total_sent
}

#[test]
fn second() {
    let input = include_str!("../test2.txt");
    let result = problem2(input);
    assert_eq!(result, 3)
}
