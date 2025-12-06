use core::panic;
use itertools::{Itertools, MinMaxResult};
use std::{collections::HashMap, fmt::Display};

use intcode::{ExecutionResult, Intcode};

fn main() {
    let input = common::read_input!();
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Intcode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "#",
                Tile::Block => "x",
                Tile::HorizontalPaddle => "_",
                Tile::Ball => "o",
            }
        )
    }
}

struct Screen {
    pixels: HashMap<(i64, i64), Tile>,
    score: i64,
}

impl Screen {
    fn new(output: &[i64]) -> Self {
        let mut screen = Screen {
            pixels: HashMap::new(),
            score: 0,
        };
        screen.update(output);
        screen
    }

    fn update(&mut self, output: &[i64]) {
        for triple in output.chunks(3) {
            let &[x, y, id] = triple else {
                panic!()
            };

            if x == -1 && y == 0 {
                self.score = id;
            } else {
                let tile: Tile = id.into();
                self.pixels.insert((x, y), tile);
            }
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let MinMaxResult::MinMax(min_x, max_x) = self.pixels.keys().map(|x| x.0).minmax() else { panic!()};
        let MinMaxResult::MinMax(min_y, max_y) = self.pixels.keys().map(|x| x.1).minmax() else { panic!()};

        writeln!(f, "Score: {}", self.score)?;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(f, "{}", self.pixels.get(&(x, y)).unwrap_or(&Tile::Empty))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

enum Joystick {
    Neutral,
    Left,
    Right,
}

impl From<Joystick> for i64 {
    fn from(value: Joystick) -> Self {
        match value {
            Joystick::Neutral => 0,
            Joystick::Left => -1,
            Joystick::Right => 1,
        }
    }
}

struct Game {
    program: Intcode,
    screen: Screen,
}

impl Game {
    fn new(mut program: Intcode) -> Self {
        program.program[0] = 2;
        program.execute();
        let screen = Screen::new(&program.output);
        Game { program, screen }
    }

    fn score(&self) -> i64 {
        self.screen.score
    }

    fn execute(&mut self) -> bool {
        let result = self.program.execute();
        self.screen.update(&self.program.output);
        // the game provides partial updates for the output, so we can clear it after every execution
        self.program.output.clear();

        result != ExecutionResult::Halted
    }

    fn move_joystick(&mut self, direction: Joystick) {
        self.program.input.push(direction.into());
    }

    fn ball_column(&self) -> i64 {
        self.screen
            .pixels
            .iter()
            .find_map(|((x, _y), v)| (*v == Tile::Ball).then_some(*x))
            .unwrap()
    }

    fn paddle_column(&self) -> i64 {
        self.screen
            .pixels
            .iter()
            .find_map(|((x, _y), v)| (*v == Tile::HorizontalPaddle).then_some(*x))
            .unwrap()
    }

    fn blocks_left(&self) -> usize {
        self.screen
            .pixels
            .values()
            .filter(|x| **x == Tile::Block)
            .count()
    }
}

fn problem1(input: &Input) -> usize {
    let mut program = Game::new(input.clone());
    program.execute();
    program.blocks_left()
}

fn problem2(input: &Input) -> i64 {
    let mut game = Game::new(input.clone());
    while game.execute() {
        // Go where the ball is
        let direction = match game.ball_column().cmp(&game.paddle_column()) {
            std::cmp::Ordering::Less => Joystick::Left,
            std::cmp::Ordering::Equal => Joystick::Neutral,
            std::cmp::Ordering::Greater => Joystick::Right,
        };

        game.move_joystick(direction);
    }

    game.score()
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 230)
    }

    #[test]
    fn second() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 11140)
    }
}
