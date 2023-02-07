use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<_> = input.lines().collect();
    let mut map = Map::new(input);
    let result = map.clone().orthogonal_overlaps();
    println!("first answer: {result}");
    let result = map.all_overlaps();
    println!("second answer: {result}");
}

#[derive(Clone, Copy)]
struct Range {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |x: &str| x.parse::<i32>())(input)
}

impl Range {
    fn new(s: &str) -> Range {
        let result = map(
            separated_pair(
                separated_pair(number, char(','), number),
                tag(" -> "),
                separated_pair(number, char(','), number),
            ),
            |((x1, y1), (x2, y2))| Range { x1, y1, x2, y2 },
        )(s);
        result.unwrap().1
    }

    fn all_coords(&self, only_orthogonal: bool) -> Vec<(i32, i32)> {
        let minx = self.x1.min(self.x2);
        let maxx = self.x1.max(self.x2);

        let miny = self.y1.min(self.y2);
        let maxy = self.y1.max(self.y2);

        let is_orthogonal = self.x1 == self.x2 || self.y1 == self.y2;
        let is_diagonal = maxx - minx == maxy - miny;

        if is_orthogonal {
            // generate an orthogonal line
            (minx..maxx + 1)
                .flat_map(|x| (miny..maxy + 1).map(move |y| (x, y)))
                .collect()
        } else if is_diagonal && !only_orthogonal {
            // generate a diagonal line
            let m = (self.y2 - self.y1) / (self.x2 - self.x1);

            (minx..maxx + 1)
                .map(|x| {
                    let y = m * (x - self.x1) + self.y1;
                    (x, y)
                })
                .collect()
        } else {
            vec![]
        }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "({}, {}) -> ({}, {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

#[derive(Clone)]
struct Map {
    cells: Vec<Vec<i32>>,
    ranges: Vec<Range>,
}
impl Map {
    fn new(coords: Vec<&str>) -> Map {
        let ranges: Vec<Range> = coords.iter().map(|pair| Range::new(pair)).collect();
        let max: usize = ranges
            .iter()
            .fold(0, |acc, r| r.x1.max(r.x2).max(r.y1).max(r.y2).max(acc))
            .try_into()
            .unwrap();

        let cells = vec![vec![0; max + 1]; max + 1];

        Map { cells, ranges }
    }

    fn overlaps<F>(&mut self, f: F) -> i32
    where
        F: Fn(&Range) -> Vec<(i32, i32)>,
    {
        for r in &self.ranges {
            for (x, y) in f(r) {
                let x: usize = x.try_into().unwrap();
                let y: usize = y.try_into().unwrap();
                self.cells[y][x] += 1;
            }
        }

        self.cells
            .iter()
            .flatten()
            .filter(|c| **c > 1)
            .count()
            .try_into()
            .unwrap()
    }

    fn all_overlaps(&mut self) -> i32 {
        self.overlaps(|r| r.all_coords(false))
    }
    fn orthogonal_overlaps(&mut self) -> i32 {
        self.overlaps(|r| r.all_coords(true))
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in self.cells.iter() {
            for value in col.iter() {
                let value = if *value == 0 {
                    ".".to_string()
                } else {
                    value.to_string()
                };
                write!(f, "{value} ").expect("couldn't write a cell");
            }
            writeln!(f, " ").expect("couldn't write the newline");
        }

        f.write_str("")
    }
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let input: Vec<_> = input.lines().collect();
    let mut map = Map::new(input);
    let result = map.orthogonal_overlaps();
    println!("{map}");
    assert_eq!(5, result);
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let input: Vec<_> = input.lines().collect();
    let mut map = Map::new(input);
    let result = map.all_overlaps();
    println!("{map}");
    assert_eq!(12, result);
}

#[test]
fn lines() {
    let r1 = Range::new("1,1 -> 1,3");
    let line1 = r1.all_coords(true);
    assert_eq!(line1, vec![(1, 1), (1, 2), (1, 3)]);

    let r2 = Range::new("9,7 -> 7,7");
    let line2 = r2.all_coords(true);
    assert_eq!(line2, vec![(7, 7), (8, 7), (9, 7)]);

    let r3 = Range::new("9,7 -> 7,9");
    let line3 = r3.all_coords(false);
    assert_eq!(line3, vec![(7, 9), (8, 8), (9, 7)]);
}
