use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
}

type Input = Vec<Point>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point((i32, i32, i32, i32));

impl Point {
    fn manhattan(&self, other: &Point) -> u32 {
        let (sx, sy, sz, sa) = self.0;
        let (ox, oy, oz, oa) = other.0;

        sx.abs_diff(ox) + sy.abs_diff(oy) + sz.abs_diff(oz) + sa.abs_diff(oa)
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(separated_list1(tag(","), i32), |x| {
            Point((x[0], x[1], x[2], x[3]))
        }),
    )(input);

    result.unwrap().1
}

pub struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind((0..size).collect())
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut y = self.0[x];
        if y != x {
            y = self.find(y);
        }
        y
    }

    pub fn union(&mut self, idx: usize, idy: usize) {
        let x = self.find(idx);
        let y = self.find(idy);
        self.0[y] = x;
    }

    pub fn sets(&mut self) -> usize {
        let mut s: HashSet<usize> = HashSet::new();
        for i in 0..self.0.len() {
            s.insert(self.find(i));
        }
        s.len()
    }
}

fn problem1(input: &Input) -> usize {
    let map: HashMap<&Point, usize> = input.iter().enumerate().map(|(i, p)| (p, i)).collect();

    let mut uf = UnionFind::new(input.len());

    for (p1, &i1) in &map {
        for (p2, &i2) in &map {
            if p1.manhattan(p2) <= 3 {
                uf.union(i1, i2);
            }
        }
    }

    uf.sets()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let cases = [
            (include_str!("../test1.txt"), 2),
            (include_str!("../test2.txt"), 4),
            (include_str!("../test3.txt"), 3),
            (include_str!("../test4.txt"), 8),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem1(&input);
            assert_eq!(result, expected)
        }
    }
}
