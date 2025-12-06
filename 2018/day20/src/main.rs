use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

#[derive(Debug)]
enum Step {
    Dir(Direction),
    Branch(Vec<Vec<Step>>),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn move_room(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
}

type DistanceMap = HashMap<(usize, usize), usize>;
type Input = Vec<Step>;

fn direction(s: &str) -> IResult<&str, Step> {
    map(
        alt((
            map(char('N'), |_| Direction::North),
            map(char('S'), |_| Direction::South),
            map(char('E'), |_| Direction::East),
            map(char('W'), |_| Direction::West),
        )),
        Step::Dir,
    ).parse(s)
}

fn branches(s: &str) -> IResult<&str, Step> {
    map(
        delimited(char('('), separated_list1(char('|'), opt(path)), char(')')),
        |x| Step::Branch(x.into_iter().flatten().collect()),
    ).parse(s)
}

fn path(s: &str) -> IResult<&str, Vec<Step>> {
    many1(alt((direction, branches))).parse(s)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = delimited(char('^'), path, char('$')).parse(input);

    result.unwrap().1
}

fn create_map(
    mut from: (usize, usize),
    mut distance: usize,
    steps: &[Step],
    distances: &mut DistanceMap,
) {
    for step in steps {
        match step {
            Step::Dir(d) => {
                let to = d.move_room(from);
                // we went through a door
                distance += 1;

                // get the adjacency list from where we're at
                distances
                    .entry(to)
                    .and_modify(|x| *x = (*x).min(distance))
                    .or_insert(distance);

                from = to;
            }
            Step::Branch(branches) => {
                for branch in branches {
                    create_map(from, distance, branch, distances);
                }
            }
        }
    }
}

fn problem(input: &Input) -> (usize, usize) {
    // build up the map from the steps, start from 5000, 5000 so we don't have to go in and out of usize's or risk underflow
    let mut distances: DistanceMap = HashMap::new();
    create_map((5000, 5000), 0, input, &mut distances);

    let answer1 = *distances.values().max().unwrap();
    let answer2 = distances.values().filter(|x| **x >= 1000).count();
    (answer1, answer2)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let cases = [
            ("^WNE$", 3),
            ("^ENWWW(NEEE|SSE(EE|N))$", 10),
            ("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
            ("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
            (
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
                31,
            ),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let (result, _result2) = problem(&input);
            assert_eq!(result, expected);
        }
    }
}
