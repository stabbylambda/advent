use std::collections::BTreeSet;

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

type Input = BTreeSet<Point3>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(newline, many1(alt((char('#'), char('.'))))),
        |input| {
            input
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, cell)| {
                        (*cell == '#').then_some((0_i64, y as i64, x as i64))
                    })
                })
                .collect()
        },
    )(input);

    result.unwrap().1
}

trait Point {
    fn neighbors(p: &Self) -> BTreeSet<Self>
    where
        Self: Sized;
}

type Point3 = (i64, i64, i64);
impl Point for Point3 {
    fn neighbors((z, y, x): &Self) -> BTreeSet<Self>
    where
        Self: Sized,
    {
        (-1..=1)
            .flat_map(|dz| {
                (-1..=1).flat_map(move |dy| {
                    (-1..=1).filter_map(move |dx| {
                        (!(dx == 0 && dy == 0 && dz == 0)).then_some((z + dz, y + dy, x + dx))
                    })
                })
            })
            .collect()
    }
}

type Point4 = (i64, i64, i64, i64);
impl Point for Point4 {
    fn neighbors((w, z, y, x): &Self) -> BTreeSet<Self>
    where
        Self: Sized,
    {
        (-1..=1)
            .flat_map(|dw| {
                (-1..=1).flat_map(move |dz| {
                    (-1..=1).flat_map(move |dy| {
                        (-1..=1).filter_map(move |dx| {
                            (!(dw == 0 && dx == 0 && dy == 0 && dz == 0)).then_some((
                                w + dw,
                                z + dz,
                                y + dy,
                                x + dx,
                            ))
                        })
                    })
                })
            })
            .collect()
    }
}

fn tick<T: Ord + Clone + Point>(points: &BTreeSet<T>) -> BTreeSet<T> {
    // consider all the points we have plus all the points that are neighbors of points we have
    // everything else is inactive and will remain so
    let mut to_consider = points.clone();
    for point in points {
        to_consider.extend(<T as Point>::neighbors(point))
    }

    to_consider
        .into_iter()
        .filter_map(|point| {
            let was_active = points.contains(&point);
            let active_neighbors = points
                .intersection(&<T as Point>::neighbors(&point))
                .count();

            let active = matches!(
                (was_active, active_neighbors),
                (true, 2) | (true, 3) | (false, 3)
            );

            active.then_some(point)
        })
        .collect()
}

fn problem1(input: &Input) -> usize {
    let mut points = input.clone();

    for _n in 0..6 {
        points = tick(&points);
    }

    points.len()
}

fn problem2(input: &Input) -> usize {
    let mut points: BTreeSet<Point4> = input.iter().map(|(z, y, x)| (0, *z, *y, *x)).collect();

    for _n in 0..6 {
        points = tick(&points);
    }

    points.len()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 112)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 848)
    }
}
