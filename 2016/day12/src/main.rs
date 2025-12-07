use common::{answer, read_input};
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i32 as nom_i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Instruction>;

#[derive(Debug)]
enum Value {
    Literal(i32),
    Register(Register),
}

type Register = char;

#[derive(Debug)]
enum Instruction {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Value, i32),
}

fn parse(input: &str) -> Input {
    let value = |s| alt((map(nom_i32, Value::Literal), map(anychar, Value::Register))).parse(s);

    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(preceded(tag("inc "), anychar), Instruction::Increment),
            map(preceded(tag("dec "), anychar), Instruction::Decrement),
            map(
                preceded(tag("cpy "), separated_pair(value, tag(" "), anychar)),
                |(v, r)| Instruction::Copy(v, r),
            ),
            map(
                preceded(tag("jnz "), separated_pair(value, tag(" "), nom_i32)),
                |(r, o)| Instruction::JumpNotZero(r, o),
            ),
        )),
    ).parse(input);

    result.unwrap().1
}

fn compute(input: &Input, a: i32, b: i32, c: i32, d: i32) -> (i32, i32, i32, i32) {
    let mut registers = HashMap::new();
    registers.insert('a', a);
    registers.insert('b', b);
    registers.insert('c', c);
    registers.insert('d', d);

    let mut current = 0i32;
    while let Some(instruction) = input.get(current as usize) {
        current += match instruction {
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
            Instruction::JumpNotZero(v, o) => {
                let v = match *v {
                    Value::Literal(x) => x,
                    Value::Register(r) => *registers.get(&r).unwrap(),
                };

                if v != 0 {
                    *o
                } else {
                    1
                }
            }
        };
    }

    (
        registers[&'a'],
        registers[&'b'],
        registers[&'c'],
        registers[&'d'],
    )
}

fn problem1(input: &Input) -> i32 {
    compute(input, 0, 0, 0, 0).0
}

fn problem2(input: &Input) -> i32 {
    compute(input, 0, 0, 1, 0).0
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 42)
    }
}
