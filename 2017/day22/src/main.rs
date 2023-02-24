use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
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

type Input = Vec<Vec<bool>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((map(char('#'), |_| true), map(char('.'), |_| false)))),
    )(input);

    result.unwrap().1
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}

fn problem1(input: &Input) -> u32 {
    let bursts = 10_000;
    let mut infection_count = 0;
    let mut infected_nodes: HashSet<(i64, i64)> = HashSet::new();
    let mut dir = Direction::Up;

    // we start in the middle of the grid for ease of computation
    let mut current = (input.len() as i64 / 2, input.len() as i64 / 2);
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell {
                infected_nodes.insert((x as i64, y as i64));
            }
        }
    }

    for _n in 0..bursts {
        let current_infected = infected_nodes.contains(&current);
        // execute the turn
        dir = match current_infected {
            true => dir.turn_right(),
            false => dir.turn_left(),
        };

        if current_infected {
            infected_nodes.remove(&current);
        } else {
            // keep track of all the times we've infected a node
            infection_count += 1;
            infected_nodes.insert(current);
        }

        match dir {
            Direction::Up => current.1 -= 1,
            Direction::Down => current.1 += 1,
            Direction::Left => current.0 -= 1,
            Direction::Right => current.0 += 1,
        };
    }

    infection_count
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
        assert_eq!(result, 5587)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
