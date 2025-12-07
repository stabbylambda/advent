use std::ops::RangeInclusive;

use common::{answer, extensions::RangeExt, read_input};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Claim>;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn range_y(&self) -> RangeInclusive<usize> {
        self.y..=(self.y + self.height - 1)
    }
    fn range_x(&self) -> RangeInclusive<usize> {
        self.x..=(self.x + self.width - 1)
    }

    fn overlaps(&self, other: &Claim) -> bool {
        let overlaps_y = self.range_y().partially_contains(&other.range_y())
            || other.range_y().partially_contains(&self.range_y());
        let overlaps_x = self.range_x().partially_contains(&other.range_x())
            || other.range_x().partially_contains(&self.range_x());

        overlaps_y && overlaps_x
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            (
                preceded(tag("#"), u32),
                preceded(tag(" @ "), u32),
                preceded(tag(","), u32),
                preceded(tag(": "), u32),
                preceded(tag("x"), u32),
            ),
            |(id, x, y, width, height)| Claim {
                id,
                x: x as usize,
                y: y as usize,
                width: width as usize,
                height: height as usize,
            },
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let mut fabric = vec![vec![0; 2000]; 2000];

    for claim in input {
        for y in claim.range_y() {
            for x in claim.range_x() {
                fabric[y][x] += 1;
            }
        }
    }

    fabric.iter().flatten().filter(|cell| **cell > 1).count()
}

fn problem2(input: &Input) -> u32 {
    let intact: Vec<&Claim> = input
        .iter()
        .filter(|claim1| {
            let no_overlaps = input
                .iter()
                .filter(|x| x.id != claim1.id)
                .all(|claim2| !claim1.overlaps(claim2));
            no_overlaps
        })
        .collect();

    assert!(intact.len() == 1);

    intact[0].id
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 4)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 3)
    }
}
