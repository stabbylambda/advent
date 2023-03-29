use std::fmt::Display;

use common::map::Map;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Scaffold,
    Space,
    Robot(Direction),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Scaffold => '#',
            Tile::Space => '.',
            Tile::Robot(Direction::Up) => '^',
            Tile::Robot(Direction::Left) => '<',
            Tile::Robot(Direction::Right) => '>',
            Tile::Robot(Direction::Down) => 'v',
        };

        write!(f, "{c}")
    }
}

fn output_to_map(output: &[i64]) -> Map<Tile> {
    let s: String = output.iter().map(|x| (*x as u8) as char).collect();
    let v: Vec<Vec<Tile>> = s
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    '.' => Tile::Space,
                    '#' => Tile::Scaffold,
                    '^' => Tile::Robot(Direction::Up),
                    '<' => Tile::Robot(Direction::Left),
                    '>' => Tile::Robot(Direction::Right),
                    'v' => Tile::Robot(Direction::Down),
                    _ => unreachable!(),
                })
                .collect::<Vec<Tile>>()
        })
        .filter(|x| !x.is_empty())
        .collect();

    Map::new(v)
}

fn problem1(input: &Input) -> usize {
    let mut program = input.clone();
    program.execute();
    let m = output_to_map(&program.output);

    m.into_iter()
        .filter(|x| {
            x.data == &Tile::Scaffold
                && x.neighbors().into_iter().all(|n| n.data == &Tile::Scaffold)
        })
        .map(|x| x.coords.0 * x.coords.1)
        .sum()
}

/* I did this by hand, no shame.
The whole map (mine, anyway) is traversed with this set of steps:
L,6,R,12,L,6,L,8,L,8,L,6,R,12,L,6,L,8,L,8,L,6,R,12,R,8,L,8,L,4,L,4,L,6,L,6,R,12,R,8,L,8,L,6,R,12,L,6,L,8,L,8,L,4,L,4,L,6,L,6,R,12,R,8,L,8,L,4,L,4,L,6,L,6,R,12,L,6,L,8,L,8

That breakdown (by examining longest substrings) looks like this:
L,6,R,12,L,6,L,8,L,8
L,6,R,12,L,6,L,8,L,8
L,6,R,12,R,8,L,8
L,4,L,4,L,6
L,6,R,12,R,8,L,8
L,6,R,12,L,6,L,8,L,8
L,4,L,4,L,6
L,6,R,12,R,8,L,8
L,4,L,4,L,6
L,6,R,12,L,6,L,8,L,8

So that means that:
L,6,R,12,L,6,L,8,L,8
L,6,R,12,R,8,L,8
L,4,L,4,L,6

And the overall program is:

A,A,B,C,B,A,C,B,C,A

and of course we don't want a continuous video feed, so...a final `n`

*/
fn problem2(input: &Input) -> i64 {
    let mut program = input.clone();
    program.program[0] = 2;
    let bot_program = "A,A,B,C,B,A,C,B,C,A
L,6,R,12,L,6,L,8,L,8
L,6,R,12,R,8,L,8
L,4,L,4,L,6
n
";

    // gotta reverse it because the program pops off the end
    program.input = bot_program.chars().rev().map(|x| x as i64).collect();
    program.execute();
    program.get_last_output()
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
        assert_eq!(result, 7404);
    }

    #[test]
    fn second() {
        let input = include_str!("../input.txt");
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 929045)
    }
}
