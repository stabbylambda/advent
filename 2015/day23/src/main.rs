use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i32 as offset, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(i32),
    JumpIfEven(char, i32),
    JumpIfOne(char, i32),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(preceded(tag("hlf "), anychar), Instruction::Half),
            map(preceded(tag("tpl "), anychar), Instruction::Triple),
            map(preceded(tag("inc "), anychar), Instruction::Increment),
            map(preceded(tag("jmp "), offset), Instruction::Jump),
            map(
                preceded(tag("jie "), separated_pair(anychar, tag(", "), offset)),
                |(r, o)| Instruction::JumpIfEven(r, o),
            ),
            map(
                preceded(tag("jio "), separated_pair(anychar, tag(", "), offset)),
                |(r, o)| Instruction::JumpIfOne(r, o),
            ),
        )),
    ).parse(input);

    result.unwrap().1
}

fn compute(input: &Input, a: u32, b: u32) -> (u32, u32) {
    let mut registers = HashMap::new();
    registers.insert('a', a);
    registers.insert('b', b);

    let mut current = 0i32;
    while let Some(instruction) = input.get(current as usize) {
        current += match instruction {
            Instruction::Half(r) => {
                registers.entry(*r).and_modify(|x| *x /= 2);
                1
            }
            Instruction::Triple(r) => {
                registers.entry(*r).and_modify(|x| *x *= 3);
                1
            }
            Instruction::Increment(r) => {
                registers.entry(*r).and_modify(|x| *x += 1);
                1
            }
            Instruction::Jump(o) => *o,
            Instruction::JumpIfEven(r, o) => {
                if registers[r] % 2 == 0 {
                    *o
                } else {
                    1
                }
            }
            Instruction::JumpIfOne(r, o) => {
                if registers[r] == 1 {
                    *o
                } else {
                    1
                }
            }
        };
    }

    (registers[&'a'], registers[&'b'])
}

fn problem1(input: &Input) -> u32 {
    compute(input, 0, 0).1
}

fn problem2(input: &Input) -> u32 {
    compute(input, 1, 0).1
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 184)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 231)
    }
}
