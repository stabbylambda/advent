use common::extensions::PointExt;
use common::heading::Heading;
use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i64 as nom_i64},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
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

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}
type Input = Vec<(Instruction, i64)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag(", "),
        alt((
            map(preceded(char('L'), nom_i64), |x| (Instruction::Left, x)),
            map(preceded(char('R'), nom_i64), |x| (Instruction::Right, x)),
        )),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> i64 {
    let mut position = (Heading::North, 0i64, 0i64);
    for (turn, blocks) in input {
        let (heading, x, y) = position;
        let new_heading = match turn {
            Instruction::Left => heading.turn_left(),
            Instruction::Right => heading.turn_right(),
        };

        let new_position = match new_heading {
            Heading::North => (x, y + blocks),
            Heading::South => (x, y - blocks),
            Heading::East => (x + blocks, y),
            Heading::West => (x - blocks, y),
        };

        position = (new_heading, new_position.0, new_position.1);
    }

    (0, 0).manhattan(&(position.1, position.2))
}

fn problem2(input: &Input) -> i64 {
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    seen.insert((0, 0));

    let mut heading = Heading::North;
    let mut position = (0i64, 0i64);
    'outer: for (turn, blocks) in input {
        let (x, y) = position;
        heading = match turn {
            Instruction::Left => heading.turn_left(),
            Instruction::Right => heading.turn_right(),
        };

        for d in 1..=*blocks {
            position = match heading {
                Heading::North => (x, y + d),
                Heading::South => (x, y - d),
                Heading::East => (x + d, y),
                Heading::West => (x - d, y),
            };

            if !seen.insert(position) {
                break 'outer;
            }
        }
    }

    (0, 0).manhattan(&position)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let result = problem1(&parse("R5, L5, R5, R3"));
        assert_eq!(result, 12);

        let result = problem1(&parse("R2, R2, R2"));
        assert_eq!(result, 2);

        let result = problem1(&parse("R2, L3"));
        assert_eq!(result, 5);
    }

    #[test]
    fn second() {
        let result = problem2(&parse("R8, R4, R4, R8"));
        assert_eq!(result, 4);
    }
}
