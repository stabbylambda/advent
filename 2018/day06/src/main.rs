use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input, 10_000);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Point>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_pair(i64, tag(", "), i64))(input);

    result.unwrap().1
}

type Point = (i64, i64);
trait PointExt {
    fn manhattan(&self, p: &Point) -> u64;
}
impl PointExt for Point {
    fn manhattan(&self, (x2, y2): &Point) -> u64 {
        let (x1, y1) = self;
        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }
}

#[derive(Debug)]
enum Area {
    Infinite,
    Finite(u64),
}

fn problem1(input: &Input) -> u64 {
    let margin = 200;

    let min_x = input.iter().map(|x| x.0).min().unwrap() - margin;
    let max_x = input.iter().map(|x| x.0).max().unwrap() + margin;
    let min_y = input.iter().map(|x| x.1).min().unwrap() - margin;
    let max_y = input.iter().map(|x| x.1).max().unwrap() + margin;

    (min_y..=max_y)
        .cartesian_product(min_x..=max_x)
        .filter_map(|p| {
            // get the single closest coordinate, ties are an error
            input
                .iter()
                .min_set_by_key(|coord| coord.manhattan(&p))
                .into_iter()
                .at_most_one()
                .unwrap_or_default()
                .map(|c| (c, p))
        })
        // group by the closest coordinate
        .into_group_map()
        .values()
        .map(|v| {
            // get the area of all the points closest to this coordinate
            let area = v.iter().fold(Area::Finite(0), |acc, &(x, y)| {
                match acc {
                    Area::Infinite => Area::Infinite,
                    // if we're at the edge, just mark it as infinite
                    Area::Finite(_) if x == min_x || x == max_x || y == min_y || y == max_y => {
                        Area::Infinite
                    }
                    // Add one to the finite region
                    Area::Finite(x) => Area::Finite(x + 1),
                }
            });

            // infinite items just don't count
            match area {
                Area::Infinite => u64::MIN,
                Area::Finite(x) => x,
            }
        })
        .max()
        .unwrap()
}

fn problem2(input: &Input, limit: u64) -> usize {
    let margin = 200;

    let min_x = input.iter().map(|x| x.0).min().unwrap() - margin;
    let max_x = input.iter().map(|x| x.0).max().unwrap() + margin;
    let min_y = input.iter().map(|x| x.1).min().unwrap() - margin;
    let max_y = input.iter().map(|x| x.1).max().unwrap() + margin;

    (min_y..=max_y)
        .cartesian_product(min_x..=max_x)
        .map(|p| input.iter().map(|coord| coord.manhattan(&p)).sum::<u64>())
        .filter(|size| *size < limit)
        .count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 17)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input, 32);
        assert_eq!(result, 16)
    }
}
