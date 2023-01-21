use std::fmt::Display;

use common::get_raw_input;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

enum Instruction {
    Up,
    Down,
    Left,
    Right,
}
type Input = Vec<Vec<Instruction>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(char('U'), |_| Instruction::Up),
            map(char('D'), |_| Instruction::Down),
            map(char('L'), |_| Instruction::Left),
            map(char('R'), |_| Instruction::Right),
        ))),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    let mut code = 0;
    let mut position = 5;

    for (idx, line) in input.iter().enumerate() {
        position = line
            .iter()
            .fold(position, |position, instruction| match instruction {
                Instruction::Up if position == 1 || position == 2 || position == 3 => position,
                Instruction::Down if position == 7 || position == 8 || position == 9 => position,
                Instruction::Left if position == 1 || position == 4 || position == 7 => position,
                Instruction::Right if position == 3 || position == 6 || position == 9 => position,

                Instruction::Up => position - 3,
                Instruction::Down => position + 3,
                Instruction::Left => position - 1,
                Instruction::Right => position + 1,
            });
        code += 10u32.pow((input.len() - idx - 1) as u32) * position;
    }

    code
}

enum Keypad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl Display for Keypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keypad::One => write!(f, "1"),
            Keypad::Two => write!(f, "2"),
            Keypad::Three => write!(f, "3"),
            Keypad::Four => write!(f, "4"),
            Keypad::Five => write!(f, "5"),
            Keypad::Six => write!(f, "6"),
            Keypad::Seven => write!(f, "7"),
            Keypad::Eight => write!(f, "8"),
            Keypad::Nine => write!(f, "9"),
            Keypad::A => write!(f, "A"),
            Keypad::B => write!(f, "B"),
            Keypad::C => write!(f, "C"),
            Keypad::D => write!(f, "D"),
        }
    }
}

impl Keypad {
    fn shift(&self, instruction: &Instruction) -> Keypad {
        match (self, instruction) {
            (Keypad::One, Instruction::Down) => Self::Three,
            (Keypad::One, _) => Self::One,

            (Keypad::Two, Instruction::Right) => Self::Three,
            (Keypad::Two, Instruction::Down) => Self::Six,
            (Keypad::Two, _) => Self::Two,

            (Keypad::Three, Instruction::Up) => Self::One,
            (Keypad::Three, Instruction::Down) => Self::Seven,
            (Keypad::Three, Instruction::Left) => Self::Two,
            (Keypad::Three, Instruction::Right) => Self::Four,

            (Keypad::Four, Instruction::Down) => Self::Eight,
            (Keypad::Four, Instruction::Left) => Self::Three,
            (Keypad::Four, _) => Self::Four,

            (Keypad::Five, Instruction::Right) => Self::Six,
            (Keypad::Five, _) => Self::Five,

            (Keypad::Six, Instruction::Up) => Self::Two,
            (Keypad::Six, Instruction::Down) => Self::A,
            (Keypad::Six, Instruction::Left) => Self::Five,
            (Keypad::Six, Instruction::Right) => Self::Seven,

            (Keypad::Seven, Instruction::Up) => Self::Three,
            (Keypad::Seven, Instruction::Down) => Self::B,
            (Keypad::Seven, Instruction::Left) => Self::Six,
            (Keypad::Seven, Instruction::Right) => Self::Eight,

            (Keypad::Eight, Instruction::Up) => Self::Four,
            (Keypad::Eight, Instruction::Down) => Self::C,
            (Keypad::Eight, Instruction::Left) => Self::Seven,
            (Keypad::Eight, Instruction::Right) => Self::Nine,

            (Keypad::Nine, Instruction::Left) => Self::Eight,
            (Keypad::Nine, _) => Self::Nine,

            (Keypad::A, Instruction::Up) => Self::Six,
            (Keypad::A, Instruction::Right) => Self::B,
            (Keypad::A, _) => Self::A,

            (Keypad::B, Instruction::Up) => Self::Seven,
            (Keypad::B, Instruction::Down) => Self::D,
            (Keypad::B, Instruction::Left) => Self::A,
            (Keypad::B, Instruction::Right) => Self::C,

            (Keypad::C, Instruction::Up) => Self::Eight,
            (Keypad::C, Instruction::Left) => Self::B,
            (Keypad::C, _) => Self::C,

            (Keypad::D, Instruction::Up) => Self::B,
            (Keypad::D, _) => Self::D,
        }
    }
}

fn problem2(input: &Input) -> String {
    let mut code = String::new();
    let mut position = Keypad::Five;

    for line in input.iter() {
        position = line.iter().fold(position, |position, instruction| {
            position.shift(instruction)
        });
        code = format!("{code}{}", position.to_string());
    }

    code
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 1985)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, "5DB3")
    }
}
