use std::collections::HashSet;

use common::get_raw_input;
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
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Point = (i64, i64);
trait PointExt {
    fn manhattan(&self, p: &Point) -> i64;
}
impl PointExt for Point {
    fn manhattan(&self, (x2, y2): &Point) -> i64 {
        let (x1, y1) = self;
        (x1.abs_diff(*x2) + y1.abs_diff(*y2)) as i64
    }
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

#[derive(Clone, Copy, Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}

impl Heading {
    fn turn(&self, turn: &Instruction) -> Heading {
        match (turn, self) {
            (Instruction::Left, Heading::North) => Heading::West,
            (Instruction::Left, Heading::West) => Heading::South,
            (Instruction::Left, Heading::South) => Heading::East,
            (Instruction::Left, Heading::East) => Heading::North,
            (Instruction::Right, Heading::North) => Heading::East,
            (Instruction::Right, Heading::East) => Heading::South,
            (Instruction::Right, Heading::South) => Heading::West,
            (Instruction::Right, Heading::West) => Heading::North,
        }
    }
}

fn problem1(input: &Input) -> i64 {
    let mut position = (Heading::North, 0i64, 0i64);
    for (turn, blocks) in input {
        let (heading, x, y) = position;
        let new_heading = heading.turn(turn);

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
        heading = heading.turn(turn);

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
    use common::test::get_raw_input;

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
