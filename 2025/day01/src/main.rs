use nom::{
    branch::alt,
    character::complete::{char, i32, newline},
    combinator::map,
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<i32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            (alt((char('L'), char('R'))), i32),
            |(dir, dist)| match dir {
                'L' => -dist,
                'R' => dist,
                _ => unreachable!(),
            },
        ),
    )
    .parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> i32 {
    let finish = x.iter().fold((0, 50i32), |(zeros, acc), x| {
        let new = (acc + x).rem_euclid(100);

        let zeros = match new {
            0 => zeros + 1,
            _ => zeros,
        };

        (zeros, new)
    });

    finish.0
}

fn problem2(x: &Input) -> i32 {
    let finish = x.iter().fold((0, 50i32), |(zeros, acc), x| {
        let new = (acc + x).rem_euclid(100);
        let rotations = (acc + x).div_euclid(100).abs();

        let mut crossings = zeros + rotations;

        if new == 0 {
            crossings += 1;
        }

        if new == 0 && x.is_positive() {
            crossings -= 1;
        }

        if acc == 0 && x.is_negative() {
            crossings -= 1;
        }

        (crossings, new)
    });

    finish.0
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn gotcha() {
        let input = "R1000";
        let input = parse(input);
        let result = problem2(&input);

        assert_eq!(result, 10)
    }

    #[test]
    fn gotcha2() {
        let input = "L49\nL2";
        let input = parse(input);
        let result = problem2(&input);

        assert_eq!(result, 1)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 6)
    }
}
