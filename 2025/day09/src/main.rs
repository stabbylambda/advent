use common::{answer, read_input};
use itertools::Itertools;
use nom::{
    bytes::tag,
    character::complete::{newline, usize},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<(usize, usize)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_pair(usize, tag(","), usize)).parse(input);

    result.unwrap().1
}

struct Rectangle {
    min: (usize, usize),
    max: (usize, usize),
}

impl Rectangle {
    fn new(a: (usize, usize), b: (usize, usize)) -> Self {
        let (ax, ay) = a;
        let (bx, by) = b;

        let (min_x, max_x) = (ax.min(bx), ax.max(bx));
        let (min_y, max_y) = (ay.min(by), ay.max(by));

        Self {
            min: (min_x, min_y),
            max: (max_x, max_y),
        }
    }

    fn area(&self) -> usize {
        let (ax, ay) = self.min;
        let (bx, by) = self.max;

        let x = ax.abs_diff(bx) + 1;
        let y = ay.abs_diff(by) + 1;

        x * y
    }

    fn enclosed_in(&self, edges: &[Rectangle]) -> bool {
        let (txmin, tymin) = &self.min;
        let (txmax, tymax) = &self.max;

        !edges.iter().any(|r| {
            let (exmin, eymin) = &r.min;
            let (exmax, eymax) = &r.max;

            exmax > txmin && exmin < txmax && eymax > tymin && eymin < tymax
        })
    }
}

fn problem1(x: &Input) -> usize {
    x.iter()
        .combinations(2)
        .map(|x| Rectangle::new(*x[0], *x[1]))
        .map(|r| r.area())
        .max()
        .unwrap()
}

fn problem2(x: &Input) -> usize {
    let mut x = x.clone();
    x.push(x[0]);

    let edges = x
        .windows(2)
        .map(|pair| Rectangle::new(pair[0], pair[1]))
        .collect_vec();

    let tiles = x
        .iter()
        .combinations(2)
        .map(|x| Rectangle::new(*x[0], *x[1]))
        .collect_vec();

    let result = tiles
        .iter()
        .filter(|t| t.enclosed_in(&edges))
        .map(|t| t.area())
        .max()
        .unwrap();

    result
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 50);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 24)
    }
}
