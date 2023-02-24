use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let mut input = parse(input);

    let answer = problem1(&mut input.clone());
    println!("problem 1 answer: {answer}");

    let answer = problem2(&mut input);
    println!("problem 2 answer: {answer}");
}

type Input = InfectedNodes;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((map(char('#'), |_| true), map(char('.'), |_| false)))),
        ),
        InfectedNodes::new,
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    const STATES: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    fn index(&self) -> usize {
        *self as usize
    }

    fn turn_right(&self) -> Self {
        Self::STATES[(self.index() + 1) % 4]
    }

    fn reverse(&self) -> Self {
        Self::STATES[(self.index() + 2) % 4]
    }

    fn turn_left(&self) -> Self {
        Self::STATES[(self.index() + 3) % 4]
    }
}

#[derive(Clone, Copy)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Clone)]
struct InfectedNodes {
    current: (i64, i64),
    dir: Direction,
    nodes: HashMap<(i64, i64), Node>,
}

impl InfectedNodes {
    fn new(input: Vec<Vec<bool>>) -> Self {
        let mut nodes = HashMap::new();
        // we start in the middle of the grid for ease of computation
        let current = (input.len() as i64 / 2, input.len() as i64 / 2);
        for (y, row) in input.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    nodes.insert((x as i64, y as i64), Node::Infected);
                }
            }
        }

        Self {
            current,
            nodes,
            dir: Direction::Up,
        }
    }

    fn move_dir(&mut self) {
        let (x, y) = self.current;
        self.current = match self.dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
    }
}

fn problem1(input: &mut Input) -> u32 {
    let mut infection_count = 0;

    for _n in 0..10_000 {
        let current_infected = input.nodes.entry(input.current).or_insert(Node::Clean);
        // execute the turn
        input.dir = match current_infected {
            Node::Infected => input.dir.turn_right(),
            _ => input.dir.turn_left(),
        };

        *current_infected = match current_infected {
            Node::Infected => Node::Clean,
            _ => {
                infection_count += 1;
                Node::Infected
            }
        };

        input.move_dir();
    }

    infection_count
}

fn problem2(input: &mut Input) -> u32 {
    let mut infection_count = 0;

    for _n in 0..10_000_000 {
        let current_infected = input.nodes.entry(input.current).or_insert(Node::Clean);
        // execute the turn
        input.dir = match current_infected {
            Node::Clean => input.dir.turn_left(),
            Node::Weakened => input.dir,
            Node::Infected => input.dir.turn_right(),
            Node::Flagged => input.dir.reverse(),
        };

        *current_infected = match current_infected {
            Node::Clean => Node::Weakened,
            Node::Weakened => {
                infection_count += 1;
                Node::Infected
            }
            Node::Infected => Node::Flagged,
            Node::Flagged => Node::Clean,
        };

        input.move_dir();
    }

    infection_count
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let mut input = parse(input);
        let result = problem1(&mut input);
        assert_eq!(result, 5587)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let mut input = parse(input);
        let result = problem2(&mut input);
        assert_eq!(result, 2511944)
    }
}
