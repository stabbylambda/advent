use std::collections::{BinaryHeap, HashSet};

use intcode::Intcode;

fn main() {
    let input = include_str!("../input.txt");
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Intcode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum BotDirections {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Robot {
    commands: Vec<BotDirections>,
    position: (i64, i64),
    program: Intcode,
}

impl Robot {
    fn new(input: &Intcode) -> Self {
        Robot {
            position: (0, 0),
            program: input.clone(),
            commands: vec![],
        }
    }

    fn steps(&self) -> usize {
        self.commands.len()
    }

    fn execute(&mut self, command: BotDirections) {
        self.program.output.clear();
        self.program.input.push(command as i64);
        self.commands.push(command);
        self.program.execute();

        let output = self.program.get_last_output();
        if output > 0 {
            match command {
                BotDirections::North => self.position.1 -= 1,
                BotDirections::South => self.position.1 += 1,
                BotDirections::West => self.position.0 -= 1,
                BotDirections::East => self.position.0 += 1,
            };
        }
    }
}

impl PartialOrd for Robot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Robot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps().cmp(&self.steps())
    }
}

fn problem1(input: &Input) -> usize {
    let robot = Robot::new(input);

    let mut moves: HashSet<(i64, i64, BotDirections)> = HashSet::new();

    let mut min_steps = usize::MAX;

    let mut queue = BinaryHeap::new();
    queue.push(robot);

    let mut considered = 0;
    while let Some(robot) = queue.pop() {
        considered += 1;
        if considered % 10 == 0 {
            println!("{considered} states, {} depth", queue.len());
        }
        if let Some(&move_result) = robot.program.output.last() {
            // are we at the oxygen generator?
            if move_result == 2 {
                min_steps = min_steps.min(robot.steps());
                continue;
            }

            // have we already gone past the minimum steps found so far?
            if robot.steps() > min_steps {
                continue;
            }
        }

        for command in [
            BotDirections::North,
            BotDirections::South,
            BotDirections::East,
            BotDirections::West,
        ] {
            // don't do the same thing from the same place again
            if moves.insert((robot.position.0, robot.position.1, command)) {
                let mut new_robot = robot.clone();
                new_robot.execute(command);
                queue.push(new_robot);
            }
        }
    }

    min_steps
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../input.txt");
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }

    #[test]
    fn second() {
        let input = include_str!("../input.txt");
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
