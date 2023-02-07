use std::fmt::Display;

use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use registers::{Register, Registers, Value};

pub mod registers;

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");
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
    let value = |s| {
        alt((
            map(i32, Value::Literal),
            map(anychar, |x| Value::Register(x)),
        ))(s)
    };

    let out = |s| {
        map(preceded(tag("out "), value), |x| {
            vec![Instruction::Transmit(x)]
        })(s)
    };
    let tgl = |s| {
        map(preceded(tag("tgl "), value), |x| {
            vec![Instruction::Toggle(x)]
        })(s)
    };
    let skip = |s| map(tag("skip"), |_| vec![Instruction::Skip])(s);
    let inc = |s| {
        map(preceded(tag("inc "), anychar), |x| {
            vec![Instruction::Increment(x)]
        })(s)
    };
    let dec = |s| {
        map(preceded(tag("dec "), anychar), |x| {
            vec![Instruction::Decrement(x)]
        })(s)
    };
    let cpy = |s| {
        map(
            preceded(tag("cpy "), separated_pair(value, tag(" "), anychar)),
            |(v, r)| vec![Instruction::Copy(v, r)],
        )(s)
    };
    let jnz = |s| {
        map(
            preceded(tag("jnz "), separated_pair(value, tag(" "), value)),
            |(r, o)| vec![Instruction::JumpNotZero(r, o)],
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
            |(_, a, b, c)| vec![Instruction::Multiply(a, b, c)],
        )(s)
    };

    // let opt_mul = |s| {
    //     map(
    //         tuple((
    //             terminated(cpy, newline),
    //             terminated(inc, newline),
    //             terminated(dec, newline),
    //             terminated(jnz, newline),
    //             terminated(dec, newline),
    //             jnz,
    //         )),
    //         |(x0, x1, x2, x3, x4, x5)| {
    //             let Instruction::Copy(b, c) = x0[0] else {panic!()};
    //             let Instruction::Increment(a) = x1[0] else {panic!()};
    //             let Instruction::Decrement(d) = x4[0] else {panic!()};

    //             vec![
    //                 Instruction::Multiply(a, b, Value::Register(d)),
    //                 Instruction::Skip,
    //                 Instruction::Skip,
    //                 Instruction::Skip,
    //                 Instruction::Skip,
    //                 Instruction::Skip,
    //             ]
    //         },
    //     )(s)
    // };

    let result: IResult<&str, Input> = map(
        separated_list1(newline, alt((skip, tgl, inc, dec, cpy, jnz, mul, out))),
        |v| {
            let v: Vec<Instruction> = v.into_iter().flatten().collect();
            Program::new(v)
        },
    )(input);

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
                let b = registers.resolve(b);
                let d = registers.resolve(d);

                registers.add(a, b * d);
                registers.set('c', 0);
                registers.set('d', 0);

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

fn problem1(input: &Input) -> i32 {
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
    use common::test::get_raw_input;

    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 198)
    }
}
