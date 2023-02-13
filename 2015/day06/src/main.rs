use ndarray::{s, Array2};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
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
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    from: (usize, usize),
    to: (usize, usize),
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        pair(
            alt((
                map(tag("toggle "), |_| InstructionType::Toggle),
                map(tag("turn on "), |_| InstructionType::TurnOn),
                map(tag("turn off "), |_| InstructionType::TurnOff),
            )),
            separated_pair(
                separated_pair(nom_u32, char(','), nom_u32),
                tag(" through "),
                separated_pair(nom_u32, char(','), nom_u32),
            ),
        ),
        |(instruction_type, (from, to))| Instruction {
            instruction_type,
            from: (from.0 as usize, from.1 as usize),
            to: (to.0 as usize, to.1 as usize),
        },
    )(input)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, instruction)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let mut lights = Array2::<u8>::zeros((1000, 1000));
    for i in input {
        let x_r = i.from.0..=i.to.0;
        let y_r = i.from.1..=i.to.1;
        let slice = lights.slice_mut(s![x_r, y_r]);

        for x in slice {
            match i.instruction_type {
                InstructionType::TurnOn => *x = 1,
                InstructionType::TurnOff => *x = 0,
                InstructionType::Toggle => *x = u8::from(*x == 0),
            }
        }
    }

    lights.iter().filter(|x| **x == 1).count()
}

fn problem2(input: &Input) -> i32 {
    let mut lights = Array2::<i32>::zeros((1000, 1000));
    for i in input {
        let x_r = i.from.0..=i.to.0;
        let y_r = i.from.1..=i.to.1;
        let slice = lights.slice_mut(s![x_r, y_r]);

        for x in slice {
            match i.instruction_type {
                InstructionType::TurnOn => *x += 1,
                InstructionType::TurnOff => *x = if *x == 0 { 0 } else { *x - 1 },
                InstructionType::Toggle => *x += 2,
            }
        }
    }

    lights.sum()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1_000_000)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2_000_000)
    }
}
