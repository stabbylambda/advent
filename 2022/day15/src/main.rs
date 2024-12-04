use common::extensions::PointExt;
use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input, 2_000_000);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input, 4_000_000);
    println!("problem 2 answer: {answer}");
}

#[derive(Debug)]
struct Input {
    sensors: Vec<Sensor>,
}

impl Input {
    fn get_coverages(&self, row: i64) -> Vec<(i64, i64)> {
        let mut coverages: Vec<(i64, i64)> = self
            .sensors
            .par_iter()
            .filter_map(|s| s.get_coverage(row))
            .collect();

        coverages.sort();
        coverages
    }
}

fn parse_coord(input: &str) -> IResult<&str, Point> {
    separated_pair(
        preceded(tag("x="), nom_i64),
        tag(", "),
        preceded(tag("y="), nom_i64),
    )(input)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Vec<(Point, Point)>> = separated_list1(
        newline,
        preceded(
            tag("Sensor at "),
            separated_pair(parse_coord, tag(": closest beacon is at "), parse_coord),
        ),
    )(input);

    let pairs = result.unwrap().1;
    let sensors = pairs
        .iter()
        .map(|&(s @ (sx, sy), b)| Sensor {
            x: sx,
            y: sy,
            dist: s.manhattan(&b),
        })
        .collect();

    Input { sensors }
}

type Point = (i64, i64);

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

impl Sensor {
    fn get_coverage(&self, row: i64) -> Option<(i64, i64)> {
        let &Sensor { x, y, dist } = self;
        // only consider the rows within manhattan distance of this sensor
        let y_in_range = y - dist <= row && row <= y + dist;

        y_in_range.then(|| {
            // get the vertical distance between these two points
            let y_distance = row.abs_diff(y);
            // the horizontal difference is the rest of the manhattan distance
            let remaining_x_distance = y_distance.abs_diff(dist as u64);

            let start_x = x - (remaining_x_distance as i64);
            let end_x = x + (remaining_x_distance as i64);

            (start_x, end_x)
        })
    }
}

fn get_contiguous_ranges(ranges: &[(i64, i64)]) -> Vec<(i64, i64)> {
    // there is likely a better representation for this than a vec, but the
    // acc will only ever have 0, 1, or 2 ranges
    ranges.iter().fold(vec![], |mut acc, &r @ (ra, rb)| {
        // push the first item on the stack
        match acc.pop() {
            Some(previous @ (pa, pb)) => {
                if ra <= pb {
                    acc.push((pa, rb.max(pb)))
                } else {
                    acc.push(previous);
                    acc.push(r);
                }
            }
            None => acc.push(r),
        };

        acc
    })
}

fn problem1(input: &Input, row: i64) -> i64 {
    let coverages = input.get_coverages(row);

    // smash the ranges together and get the sum of the distances between them
    let &(start, end) = get_contiguous_ranges(&coverages).first().unwrap();

    start.abs_diff(end) as i64
}

/* I completely rewrote the implementation between problem 1 and 2. My initial
implementation was to generate a set of points which worked on small inputs, and even
on the big input, it "worked" for a single row.

The better implementation only concerns itself with merging ranges and then I sped that
up by adding rayon to make things parallel. Rayon might be cheating a bit, but the
sequential program was relatively fast even without it...and rayon is super easy to add.
*/
fn problem2(input: &Input, max_search_area: i64) -> i64 {
    let (x, y) = (0..max_search_area)
        .into_iter()
        .find_map(|row| {
            // find the coverages on this particular row
            let coverages = input.get_coverages(row);

            let ranges = get_contiguous_ranges(&coverages);
            (ranges.len() > 1).then(|| {
                // the hole in between these two ranges is the x value of the beacon
                let x = ranges[0].1 + 1;
                (x, row)
            })
        })
        .unwrap();

    (x * 4_000_000) + y
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, 10);
        assert_eq!(result, 26)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input, 20);
        assert_eq!(result, 56000011)
    }
}
