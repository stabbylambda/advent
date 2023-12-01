use std::collections::BTreeSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
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

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Step = (Direction, u32);
type Input = Vec<Step>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(
            alt((
                map(char('U'), |_| Direction::Up),
                map(char('D'), |_| Direction::Down),
                map(char('L'), |_| Direction::Left),
                map(char('R'), |_| Direction::Right),
            )),
            tag(" "),
            nom_u32,
        ),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Knot((i32, i32));
impl Knot {
    /** Calculate the follower position according to these moves.

    Given:
     * L: the leader
     * 1: the possible positions for problem 1
     * 2: the possible extra position for problem 2

    We want to translate them to the following

    ```
        input        result
        21112        .....
        1...1        .212.
        1.H.1  --->  .1H1.
        1...1        .212.
        21112        .....
    ```
    */
    fn get_follower_position(&self, follower: Knot) -> Knot {
        let leader = *self;
        let Knot((lx, ly)) = leader;
        let Knot((fx, fy)) = follower;
        let (dx, dy) = (lx - fx, ly - fy);

        /* Neat, TIL about https://en.wikipedia.org/wiki/Chebyshev_distance which replaces
        the crappy manual math I had done.
         */

        Knot(if dx.abs() > 1 || dy.abs() > 1 {
            (fx + dx.signum(), fy + dy.signum())
        } else {
            (fx, fy)
        })
    }

    fn move_dir(&self, dir: &Direction) -> Knot {
        let (x, y) = self.0;
        Knot(match dir {
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        })
    }
}

const PRINT_GRID: bool = false;
const GRID_SIZE: i32 = 40;
fn problem(knot_count: usize, input: &Input) -> usize {
    let mut points: Vec<Knot> = vec![Knot((0, 0)); knot_count];

    // keep track of all the points where the tail has been in a set
    let mut visited: BTreeSet<Knot> = BTreeSet::new();
    visited.insert(Knot((0, 0)));

    if PRINT_GRID {
        println!("== Initial ==");
        display(points.to_vec());
    }

    for (dir, count) in input {
        if PRINT_GRID {
            println!("== {dir:?} {count} ==");
        }
        for _n in 1..=*count {
            // first move the leader
            points[0] = points[0].move_dir(dir);

            // now move all the rest according to the one in front of them
            for k in 1..knot_count {
                points[k] = points[k - 1].get_follower_position(points[k]);

                // only track the tail positions
                if k == knot_count - 1 {
                    visited.insert(points[k]);
                }
            }

            if PRINT_GRID {
                display(points.to_vec());
            }
        }
    }
    visited.len()
}

// horrible hacky display code that barely works
fn display(positions: Vec<Knot>) {
    let size = GRID_SIZE;
    let (ox, oy) = (size / 2, size / 2);
    let mut grid = vec![vec![None; size as usize]; size as usize];
    for (idx, Knot((x, y))) in positions.iter().enumerate() {
        let x = (ox + *x) as usize;
        let y = (oy + *y) as usize;
        let cell = grid[y][x];

        // don't overwrite with later knots
        grid[y][x] = match cell {
            None => Some(idx),
            Some(idx) => Some(idx),
        }
    }

    for y in (0usize..size as usize).rev() {
        for x in 0usize..size as usize {
            match grid[y][x] {
                Some(0) => print!("H"),
                Some(n) if n == positions.len() - 1 => print!("T"),
                Some(n) => print!("{n}"),
                None => print!("."),
            }
        }
        println!()
    }
    println!()
}

fn problem1(input: &Input) -> usize {
    problem(2, input)
}

fn problem2(input: &Input) -> usize {
    problem(10, input)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 13)
    }

    #[test]
    fn second1() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1)
    }

    #[test]
    fn second2() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 36)
    }
}
