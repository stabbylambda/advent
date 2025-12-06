use std::collections::BTreeMap;

use intcode::{ExecutionResult, Intcode};

fn main() {
    let input = common::read_input!();
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer:\n{answer}");
}

type Input = Intcode;
#[derive(Clone, Copy, Debug)]
pub enum TurnInstruction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum CardinalHeading {
    North,
    South,
    East,
    West,
}

impl CardinalHeading {
    pub fn turn(&self, turn: &TurnInstruction) -> Self {
        match (turn, self) {
            (TurnInstruction::Left, Self::North) => Self::West,
            (TurnInstruction::Left, Self::West) => Self::South,
            (TurnInstruction::Left, Self::South) => Self::East,
            (TurnInstruction::Left, Self::East) => Self::North,
            (TurnInstruction::Right, Self::North) => Self::East,
            (TurnInstruction::Right, Self::East) => Self::South,
            (TurnInstruction::Right, Self::South) => Self::West,
            (TurnInstruction::Right, Self::West) => Self::North,
        }
    }
}

pub struct Unit(pub CardinalHeading, pub i64, pub i64);

impl Unit {
    pub fn turn_and_move(&mut self, instruction: TurnInstruction, steps: i64) {
        let Self(heading, x, y) = self;
        // make the turn
        *heading = heading.turn(&instruction);

        // move forward one
        match heading {
            CardinalHeading::North => *y -= steps,
            CardinalHeading::South => *y += steps,
            CardinalHeading::East => *x += steps,
            CardinalHeading::West => *x -= steps,
        };
    }

    pub fn position(&self) -> (i64, i64) {
        (self.1, self.2)
    }
}

fn paint(program: &Intcode, start: i64) -> BTreeMap<(i64, i64), i64> {
    let mut program = program.clone();
    let mut current: BTreeMap<(i64, i64), i64> = BTreeMap::new();

    // we're starting at 0,0
    let mut robot = Unit(CardinalHeading::North, 0i64, 0i64);
    program.input.push(start);

    while program.execute() == ExecutionResult::WaitingForInput {
        // get the last two items out of the output
        let &[color, direction] = &program.output[program.output.len() - 2..] else {
            panic!("The robot didn't provide two outputs!");
        };

        // paint the tile and mark that we've painted it
        let tile = current
            .entry(robot.position())
            .and_modify(|tile| *tile = color)
            .or_default();
        *tile = color;

        // execute the move
        let turn = match direction {
            0 => TurnInstruction::Left,
            _ => TurnInstruction::Right,
        };
        robot.turn_and_move(turn, 1);

        // get the current color (or black)
        let color = *current.get(&robot.position()).unwrap_or(&0);
        program.input.push(color);
    }

    current
}

fn problem1(input: &Input) -> usize {
    let painted = paint(input, 0);
    painted.keys().len()
}

fn problem2(input: &Input) -> String {
    let painted = paint(input, 1);
    let max_x = painted.keys().map(|x| x.0).max().unwrap();
    let max_y = painted.keys().map(|x| x.1).max().unwrap();

    let rows: Vec<String> = (0..=max_y)
        .map(|y| {
            let row: Vec<&str> = (0..=max_x)
                .map(|x| {
                    let tile = painted.get(&(x, y)).unwrap_or(&0);
                    match *tile == 0 {
                        true => " ",
                        false => "#",
                    }
                })
                .collect();
            row.join("")
        })
        .collect();

    rows.join("\n")
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::problem1;
    #[test]
    #[ignore = "input files aren't available in CI"]
    fn first() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 2041)
    }
}
