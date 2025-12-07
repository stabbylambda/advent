use std::collections::HashMap;

use common::{answer, math::lcm, read_input};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input, 1000));
    answer!(problem2(&input));
}

#[derive(Clone, Copy, Debug)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    fn new(position: (i32, i32, i32)) -> Self {
        Moon {
            position,
            velocity: (0, 0, 0),
        }
    }

    fn apply_velocity(&mut self) {
        let (vx, vy, vz) = self.velocity;
        let (x1, y1, z1) = self.position;

        self.position = (x1 + vx, y1 + vy, z1 + vz);
    }

    fn apply_gravity(&mut self, other: &Moon) {
        let (vx, vy, vz) = self.velocity;

        let (x1, y1, z1) = self.position;
        let (x2, y2, z2) = other.position;

        self.velocity = (
            vx + Self::gravity_component(x1, x2),
            vy + Self::gravity_component(y1, y2),
            vz + Self::gravity_component(z1, z2),
        );
    }

    fn energy(&self) -> i32 {
        let (x, y, z) = self.position;
        let (vx, vy, vz) = self.velocity;

        let potential = x.abs() + y.abs() + z.abs();
        let kinetic = vx.abs() + vy.abs() + vz.abs();

        potential * kinetic
    }

    fn gravity_component(c1: i32, c2: i32) -> i32 {
        match c2.cmp(&c1) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[derive(Clone, Debug)]
struct System(Vec<Moon>);

impl System {
    fn tick(&mut self) {
        let moons = &mut self.0;
        // first apply gravity
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                let mi = moons[i];
                let mj = moons[j];

                moons[i].apply_gravity(&mj);
                moons[j].apply_gravity(&mi);
            }
        }

        // now update velocity
        for x in moons.iter_mut() {
            x.apply_velocity();
        }
    }

    fn energy(&self) -> i32 {
        self.0.iter().map(|x| x.energy()).sum()
    }

    fn get_fingerprint(&self, axis: Axis) -> (Axis, Vec<i32>) {
        (
            axis,
            self.0
                .iter()
                .flat_map(|m| match axis {
                    Axis::X => vec![m.position.0, m.velocity.0],
                    Axis::Y => vec![m.position.1, m.velocity.1],
                    Axis::Z => vec![m.position.2, m.velocity.2],
                })
                .collect_vec(),
        )
    }
}

#[test]
fn velocity_test() {
    let mut europa = Moon::new((1, 2, 3));
    europa.velocity = (-2, 0, 3);
    europa.apply_velocity();
    assert_eq!(europa.position, (-1, 2, 6));
}

#[test]
fn gravity_test() {
    let mut ganymede = Moon::new((3, 1, 1));
    let mut callisto = Moon::new((5, 1, 1));

    ganymede.apply_gravity(&callisto);
    callisto.apply_gravity(&ganymede);

    assert_eq!(ganymede.velocity, (1, 0, 0));
    assert_eq!(callisto.velocity, (-1, 0, 0));
}

type Input = System;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            map(
                delimited(
                    tag("<"),
                    (
                        delimited(tag("x="), i32, tag(", ")),
                        delimited(tag("y="), i32, tag(", ")),
                        preceded(tag("z="), i32),
                    ),
                    tag(">"),
                ),
                Moon::new,
            ),
        ),
        System,
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input, count: u32) -> i32 {
    let mut moons = input.clone();
    (0..count).for_each(|_t| moons.tick());
    moons.energy()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y,
    Z,
}

fn problem2(input: &Input) -> i64 {
    let mut moons = input.clone();
    let mut seen: HashMap<(Axis, Vec<i32>), i64> = HashMap::new();
    let mut count_x: Option<i64> = None;
    let mut count_y: Option<i64> = None;
    let mut count_z: Option<i64> = None;

    /*  All the axes are independent of one another, so we can track the first repeat of each one separately.
    Once we have a repeat in each axis, we can bail and then it's the least common multiple of all three axes.
    */
    for t in 0i64.. {
        moons.tick();

        count_x = count_x.or_else(|| {
            seen.insert(moons.get_fingerprint(Axis::X), t)
                .map(|_previous| t)
        });
        count_y = count_y.or_else(|| {
            seen.insert(moons.get_fingerprint(Axis::Y), t)
                .map(|_previous| t)
        });
        count_z = count_z.or_else(|| {
            seen.insert(moons.get_fingerprint(Axis::Z), t)
                .map(|_previous| t)
        });

        if count_x.is_some() && count_y.is_some() && count_z.is_some() {
            break;
        }
    }

    lcm(count_x.unwrap(), lcm(count_y.unwrap(), count_z.unwrap()))
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, 10);
        assert_eq!(result, 179)
    }

    #[test]
    fn first_2() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem1(&input, 100);
        assert_eq!(result, 1940)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4686774924)
    }
}
