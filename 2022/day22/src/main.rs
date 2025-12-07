use common::{answer, read_input};
use common::heading::Heading;
use ndarray::prelude::*;

use crate::parsing::parse;

pub mod parsing;

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = (Array2<Space>, Vec<Instruction>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Void,
    Empty,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Walk(u32),
    TurnLeft,
    TurnRight,
}

#[allow(dead_code)]
fn print_map(grid: &Array2<Space>, player_position: &Position) {
    for (y, row) in grid.outer_iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if [y, x] == player_position.coords {
                let player_char = match player_position.heading {
                    Heading::East => ">",
                    Heading::South => "v",
                    Heading::West => "<",
                    Heading::North => "^",
                };
                print!("{player_char}");
                continue;
            }
            match space {
                Space::Void => print!(" "),
                Space::Empty => print!("."),
                Space::Wall => print!("#"),
            }
        }
        println!();
    }
    println!("==================");
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    coords: [usize; 2],
    heading: Heading,
}

impl Position {
    fn get_start(grid: &Array2<Space>) -> Position {
        // find the starting position as the first Empty square on the first row of the board
        Position {
            coords: [
                0,
                grid.row(0).iter().position(|&x| x == Space::Empty).unwrap(),
            ],
            heading: Heading::East,
        }
    }

    fn rotate(&mut self, instruction: &Instruction) {
        let heading = match instruction {
            Instruction::TurnLeft => self.heading.turn_left(),
            Instruction::TurnRight => self.heading.turn_right(),
            _ => self.heading,
        };
        self.heading = heading;
    }

    fn get_password(&self) -> u32 {
        // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
        let first = 1000 * (self.coords[0] + 1) as u32;
        let second = 4 * (self.coords[1] + 1) as u32;
        let third = match self.heading {
            Heading::East => 0,
            Heading::South => 1,
            Heading::West => 2,
            Heading::North => 3,
        };

        first + second + third
    }

    fn walk(&mut self, steps: u32, grid: &Array2<Space>, void_treatment: VoidTreatment) {
        // get the correct axis to look at and slice the array on that axis
        let (axis, idx, rev) = match self.heading {
            Heading::North => (Axis(1), 0, true),
            Heading::South => (Axis(1), 0, false),
            Heading::West => (Axis(0), 1, true),
            Heading::East => (Axis(0), 1, false),
        };

        let slice = grid.index_axis(axis, self.coords[axis.0]);

        let mut s = 0u32;
        while s < steps {
            // look at the next space
            let step = if rev { slice.len() - 1 } else { 1 };
            let next = (self.coords[idx] + step) % slice.len();

            match slice.get(next) {
                Some(Space::Wall) => break,
                Some(Space::Empty) => {
                    s += 1;
                    self.coords[idx] = next;
                }
                Some(Space::Void) => {
                    match void_treatment {
                        VoidTreatment::TwoD => {
                            let next_non_void = if rev {
                                slice
                                    .iter()
                                    .rev()
                                    .cycle()
                                    .find(|&&x| x != Space::Void)
                                    .unwrap()
                            } else {
                                slice.iter().cycle().find(|&&x| x != Space::Void).unwrap()
                            };

                            if *next_non_void == Space::Wall {
                                break;
                            }

                            // is the next non-void space a wall?
                            self.coords[idx] = next;
                        }
                        VoidTreatment::ThreeD => {
                            /* when we hit a void on the map, but the map is in 3d mode, we need to translate
                            ourselves off this cube edge and onto another, we also still need to check that
                            the next space isn't a wall */

                            // translate our coords and heading
                            let (next_row, next_col, next_heading) = self.translate_to_3d();

                            if grid[[next_row, next_col]] == Space::Wall {
                                break;
                            }

                            self.coords = [next_row, next_col];
                            self.heading = next_heading;

                            // complete the rest of the walk
                            self.walk(steps - s - 1, grid, void_treatment);
                            return;
                        }
                    }
                }
                None => unreachable!(),
            }
        }
    }

    fn get_face(&self) -> usize {
        let row = self.coords[0];
        let col = self.coords[1];

        match (row, col) {
            (0..=49, 50..=99) => 1,
            (0..=49, 100..=149) => 2,
            (50..=99, 50..=99) => 3,
            (100..=149, 50..=99) => 4,
            (100..=149, 0..=49) => 5,
            (150..=199, 0..=49) => 6,

            _ => panic!(),
        }
    }

    fn translate_to_3d(&mut self) -> (usize, usize, Heading) {
        let row = self.coords[0];
        let col = self.coords[1];

        let heading = self.heading;
        let face = self.get_face();

        match (face, heading, row, col) {
            (1, Heading::North, 0, _) => (150 + (col - 50), 0, Heading::East),
            (1, Heading::West, _, 50) => (149 - row, 0, Heading::East),

            (2, Heading::North, 0, _) => (199, col - 100, Heading::North),
            (2, Heading::East, _, 149) => (149 - row, 99, Heading::West),
            (2, Heading::South, 49, _) => (50 + (col - 100), 99, Heading::West),

            (3, Heading::West, _, 50) => (100, row - 50, Heading::South),
            (3, Heading::East, _, 99) => (49, 100 + (row - 50), Heading::North),

            (4, Heading::East, _, 99) => (49 - (row - 100), 149, Heading::West),
            (4, Heading::South, 149, _) => (150 + (col - 50), 49, Heading::West),

            (5, Heading::West, _, 0) => (49 - (row - 100), 50, Heading::East),
            (5, Heading::North, 100, _) => (50 + col, 50, Heading::East),

            (6, Heading::West, _, 0) => (0, 50 + (row - 150), Heading::South),
            (6, Heading::South, 199, _) => (0, 100 + col, Heading::South),
            (6, Heading::East, _, 49) => (149, 50 + (row - 150), Heading::North),

            _ => panic!(),
        }
    }
}

enum VoidTreatment {
    TwoD,
    ThreeD,
}

fn problem1(input: &Input) -> u32 {
    let (grid, moves) = input;
    let mut player_position = Position::get_start(grid);

    for x in moves.iter() {
        match x {
            Instruction::TurnLeft | Instruction::TurnRight => {
                player_position.rotate(x);
            }
            Instruction::Walk(steps) => player_position.walk(*steps, grid, VoidTreatment::TwoD),
        }
    }

    player_position.get_password()
}

fn problem2(input: &Input) -> u32 {
    let (grid, moves) = input;
    let mut player_position = Position::get_start(grid);

    for x in moves.iter() {
        match x {
            Instruction::TurnLeft | Instruction::TurnRight => {
                player_position.rotate(x);
            }
            Instruction::Walk(steps) => player_position.walk(*steps, grid, VoidTreatment::ThreeD),
        }
    }

    player_position.get_password()
}

#[cfg(test)]
mod test {

    use crate::{parse, parsing::parse_grid, problem1, problem2, Heading, Position};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 6032)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 5031)
    }
    #[test]
    fn walk_up() {
        let map = ". 
# 
..
.#
..";

        let grid = parse_grid(map).unwrap().1;

        let mut p = Position {
            coords: [0, 0],
            heading: Heading::North,
        };

        // can walk over the top of the map
        p.walk(5, &grid, crate::VoidTreatment::TwoD);
        assert_eq!(p.coords, [2, 0]);

        // now move over one
        p.coords = [2, 1];

        // can walk over the top, over a void, and hit a wall
        p.walk(5, &grid, crate::VoidTreatment::TwoD);
        assert_eq!(p.coords, [4, 1]);
    }

    #[test]
    fn walk_left() {
        let map = "  .#.
  ..#";

        let grid = parse_grid(map).unwrap().1;

        let mut p = Position {
            coords: [0, 2],
            heading: Heading::West,
        };

        // can walk over void left
        p.walk(3, &grid, crate::VoidTreatment::TwoD);
        assert_eq!(p.coords, [0, 4]);

        p.coords = [1, 3];
        p.walk(10, &grid, crate::VoidTreatment::TwoD);
        assert_eq!(p.coords, [1, 2]);
    }
}
