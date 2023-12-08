use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = (Vec<Direction>, HashMap<(&'a str, Direction), &'a str>);

fn parse(input: &str) -> Input {
    let instructions = many1(alt((
        map(char('L'), |_| Direction::Left),
        map(char('R'), |_| Direction::Right),
    )));

    let room = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    );
    let rooms = map(separated_list1(newline, room), create_room_map);

    let result: IResult<&str, Input> = separated_pair(instructions, tag("\n\n"), rooms)(input);

    result.unwrap().1
}

fn create_room_map<'a>(
    rooms: Vec<(&'a str, (&'a str, &'a str))>,
) -> HashMap<(&'a str, Direction), &'a str> {
    HashMap::from_iter(rooms.into_iter().flat_map(|(start, (left, right))| {
        vec![
            ((start, Direction::Left), left),
            ((start, Direction::Right), right),
        ]
    }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
}

fn problem1(input: &Input) -> u32 {
    let (directions, room_map) = input;
    let mut count = 0;
    let mut current = "AAA";
    for x in directions.iter().cycle() {
        count += 1;
        let new = room_map[&(current, *x)];
        if new == "ZZZ" {
            break;
        }

        current = new;
    }

    count
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 6)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
