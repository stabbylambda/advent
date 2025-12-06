use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = Document<'a>;

fn parse(input: &str) -> Input<'_> {
    let instructions = many1(alt((
        map(char('L'), |_| Direction::Left),
        map(char('R'), |_| Direction::Right),
    )));

    let room = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    );

    let rooms = map(separated_list1(newline, room), create_room_map);

    let result: IResult<&str, Input> = map(
        separated_pair(instructions, tag("\n\n"), rooms),
        |(directions, room_map)| Document {
            directions,
            room_map,
        },
    ).parse(input);

    result.unwrap().1
}

/** Turn the room listing into a map with direction baked into the key */
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

/** The Document in the camel's pouch */
struct Document<'a> {
    directions: Vec<Direction>,
    room_map: HashMap<(&'a str, Direction), &'a str>,
}

fn problem1(input: &Input) -> u32 {
    let mut count = 0;
    let mut current = "AAA";
    // keep going through the direction list until we find ZZZ
    for x in input.directions.iter().cycle() {
        count += 1;
        current = input.room_map[&(current, *x)];
        if current == "ZZZ" {
            break;
        }
    }

    count
}

struct Ghost<'a> {
    current: &'a str,
    document: &'a Document<'a>,
}

impl<'a> Ghost<'a> {
    fn step(&mut self, direction: &Direction) {
        self.current = self.document.room_map[&(self.current, *direction)];
    }

    fn is_at_end(&self) -> bool {
        self.current.ends_with('Z')
    }

    /** Find the count of rooms for this ghost to get to an ending room */
    fn get_cycle_time(&mut self) -> u32 {
        let mut count = 0;
        for d in self.document.directions.iter().cycle() {
            self.step(d);
            count += 1;

            if self.is_at_end() {
                break;
            }
        }

        count
    }
}

fn problem2(input: &Input) -> i64 {
    input
        .room_map
        .keys()
        .filter_map(|x| {
            // we need to create a new Ghost for every room that ends in A
            x.0.ends_with('A').then_some(Ghost {
                document: input,
                current: x.0,
            })
        })
        // have each of them find their cycle time
        .map(|mut x| x.get_cycle_time() as i64)
        // now get the least common multiple of all the different cycle times
        .reduce(common::math::lcm)
        .unwrap()
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
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 6)
    }
}
