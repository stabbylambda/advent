use common::{answer, grid::Grid, read_input};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Robot>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                preceded(tag("p="), separated_pair(i32, tag(","), i32)),
                tag(" "),
                preceded(tag("v="), separated_pair(i32, tag(","), i32)),
            ),
            |(p, v)| Robot::new(p, v),
        ),
    ).parse(input);

    result.unwrap().1
}

#[derive(Debug)]
struct Robot {
    start: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn new(start: (i32, i32), velocity: (i32, i32)) -> Self {
        Self { start, velocity }
    }

    fn position(&self, t: i32, bounds: (i32, i32)) -> (i32, i32) {
        let new_x = self.start.0 + (t * self.velocity.0);
        let new_y = self.start.1 + (t * self.velocity.1);

        let new_x = (new_x % bounds.0 + bounds.0) % bounds.0;
        let new_y = (new_y % bounds.1 + bounds.1) % bounds.1;

        (new_x, new_y)
    }
}

fn simulate(input: &Input, bounds: (i32, i32)) -> usize {
    let (hx, hy) = (bounds.0 / 2, bounds.1 / 2);
    let positions = input
        .iter()
        .map(|x| x.position(100, bounds))
        .map(|(x, y)| {
            if x < hx && y < hy {
                1
            } else if x > hx && y < hy {
                2
            } else if x < hx && y > hy {
                3
            } else if x > hx && y > hy {
                4
            } else {
                0
            }
        })
        .filter(|x| *x > 0)
        .counts();

    positions.values().product()
}

fn problem1(input: &Input) -> usize {
    let bounds = (101, 103);
    simulate(input, bounds)
}

fn has_christmas_tree_fast(robots: &[(i32, i32)]) -> bool {
    let count = robots.len();
    let distinct = robots.iter().unique().count();

    count == distinct
}

// my initial implementation was this, which is hilariously slow and bad, but was
// way better than scanning thousands of frames
#[allow(dead_code)]
fn has_christmas_tree_slow(robots: &[(i32, i32)]) -> bool {
    let mut g: Grid<char> = Grid::new(vec![vec!['.'; 101]; 103]);
    for &(x, y) in robots {
        let robot = (x as usize, y as usize);
        g.set(robot, '#');
    }
    let s = format!("{g}");
    s.contains("#########")
}
fn problem2(input: &Input) -> u32 {
    let bounds = (101, 103);

    let mut t = 0;
    loop {
        let robots = input.iter().map(|r| r.position(t, bounds)).collect_vec();
        if has_christmas_tree_fast(&robots) {
            break t as u32;
        }
        t += 1;
    }
}

#[cfg(test)]
mod test {

    use crate::{parse, simulate};
    #[test]
    fn first() {
        let bounds = (11, 7);
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = simulate(&input, bounds);
        assert_eq!(result, 12)
    }
}
