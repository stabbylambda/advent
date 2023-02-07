use std::{collections::BTreeSet};

use nom::{
    bytes::complete::tag,
    character::complete::i32 as nom_i32,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = ((i32, i32), (i32, i32));

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = preceded(
        tag("target area: "),
        separated_pair(
            preceded(tag("x="), separated_pair(nom_i32, tag(".."), nom_i32)),
            tag(", "),
            preceded(tag("y="), separated_pair(nom_i32, tag(".."), nom_i32)),
        ),
    )(input);

    result.unwrap().1
}

#[derive(PartialEq, Eq)]
enum Result {
    InProgress,
    Hit,
    Overshoot,
}

fn check_range(x: i32, y: i32, &((x1, x2), (y1, y2)): &Input) -> Result {
    let hit = x1 <= x && x <= x2 && y1 <= y && y <= y2;
    let over = y < y1;

    if hit {
        Result::Hit
    } else if over {
        Result::Overshoot
    } else {
        Result::InProgress
    }
}

fn fire(mut xv: i32, mut yv: i32, range: &Input) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;

    let mut max_y = 0;

    loop {
        match check_range(x, y, range) {
            Result::Hit => return Some(max_y),
            Result::Overshoot => return None,
            _ => {}
        };

        // increase position by velocity
        x += xv;
        y += yv;

        max_y = max_y.max(y);

        // apply drag to velocity
        xv = match xv.cmp(&0) {
            std::cmp::Ordering::Less => xv + 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => xv - 1,
        };
        yv -= 1;
    }
}

fn problem1(input: &Input) -> i32 {
    let mut max_y = 0;
    for xv in 0..200 {
        for yv in 0..200 {
            if let Some(result) = fire(xv, yv, input) {
                max_y = max_y.max(result);
            }
        }
    }
    max_y
}

fn problem2(input: &Input) -> usize {
    let mut results: BTreeSet<(i32, i32)> = BTreeSet::new();
    for xv in -400..400 {
        for yv in -400..400 {
            if fire(xv, yv, input).is_some() {
                results.insert((xv, yv));
            }
        }
    }
    dbg!(&results);
    results.len()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 45)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 112)
    }
}
