use std::ops::Neg;

use common::extensions::PointExt;
use common::heading::Heading;

use nom::{
    branch::alt,
    character::complete::{char, i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Action>;

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    TurnLeft(i32),
    TurnRight(i32),
    GoForward(i32),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(preceded(char('N'), i32), Action::North),
            map(preceded(char('S'), i32), Action::South),
            map(preceded(char('E'), i32), Action::East),
            map(preceded(char('W'), i32), Action::West),
            map(preceded(char('L'), i32), Action::TurnLeft),
            map(preceded(char('R'), i32), Action::TurnRight),
            map(preceded(char('F'), i32), Action::GoForward),
        )),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    let start = (0, 0, Heading::East);

    let end = input
        .iter()
        .fold(start, |(mut x, mut y, mut heading), action| {
            match (action, heading) {
                (Action::North(s), _) | (Action::GoForward(s), Heading::North) => y += s,
                (Action::South(s), _) | (Action::GoForward(s), Heading::South) => y -= s,
                (Action::East(s), _) | (Action::GoForward(s), Heading::East) => x += s,
                (Action::West(s), _) | (Action::GoForward(s), Heading::West) => x -= s,

                (Action::TurnLeft(d), _) => heading -= *d,
                (Action::TurnRight(d), _) => heading += *d,
            }

            (x, y, heading)
        });

    (0, 0).manhattan(&(end.0, end.1)) as u32
}

// todo: this could probably be pulled out to pointext if we need to
fn rotate((x, y): (i32, i32), degrees: i32) -> (i32, i32) {
    let (x, y) = (x as f32, y as f32);
    let (s, c) = (degrees as f32).to_radians().sin_cos();

    let new_x = (x * c - y * s).round();
    let new_y = (x * s + y * c).round();

    (new_x as i32, new_y as i32)
}

fn problem2(input: &Input) -> u32 {
    /* The waypoint is always relative to the ship, so we're basically treating it as (dx, dy).
    This makes two things easier:
        - rotations are always around (0,0), so we never have to adjust from a point to the origin and back
        - going forward doesn't change the waypoint, it's just a simple multiplication
    */
    let (_waypoint, ship) =
        input
            .iter()
            .fold(((10, 1), (0, 0)), |(mut waypoint, mut ship), action| {
                match action {
                    Action::North(s) => waypoint.1 += s,
                    Action::South(s) => waypoint.1 -= s,
                    Action::East(s) => waypoint.0 += s,
                    Action::West(s) => waypoint.0 -= s,
                    Action::GoForward(s) => {
                        ship.0 += waypoint.0 * s;
                        ship.1 += waypoint.1 * s;
                    }
                    Action::TurnLeft(d) => waypoint = rotate(waypoint, *d),
                    Action::TurnRight(d) => waypoint = rotate(waypoint, d.neg()),
                }

                (waypoint, ship)
            });

    (0, 0).manhattan(&ship) as u32
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 25)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 286)
    }
}
