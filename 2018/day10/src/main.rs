use common::{answer, read_input};
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32, newline, space0},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input, 10));
}

type Input = Vec<((i32, i32), (i32, i32))>;

fn parse(input: &str) -> Input {
    let angle_pair = |s| {
        delimited(
            char('<'),
            separated_pair(preceded(space0, i32), tag(", "), preceded(space0, i32)),
            char('>'),
        ).parse(s)
    };
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(
            preceded(tag("position="), angle_pair),
            char(' '),
            preceded(tag("velocity="), angle_pair),
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input, expected_height: u32) -> i32 {
    for t in 0.. {
        let mut current: Vec<(i32, i32)> = vec![];
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for ((x, y), (vx, vy)) in input {
            // get the points at time = t
            let x = x + t * vx;
            let y = y + t * vy;
            current.push((x, y));

            // keep track of the bounding box as we go
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        let height = min_y.abs_diff(max_y);

        if height <= expected_height {
            // I'm not writing some godawful OCR-like code to recognize ASCII art, just print the string, return time = t
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if current.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }

            return t;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, 8);
        assert_eq!(result, 3)
    }
}
