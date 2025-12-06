use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();

    let score = problem1(&parse_races(input));
    println!("problem 1 score: {score}");

    let score = problem2(&parse_single_race(input));
    println!("problem 2 score: {score}");
}

type Input = Vec<Race>;

fn parse_races(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            preceded(
                terminated(tag("Time:"), space0),
                separated_list1(space1, u32),
            ),
            newline,
            preceded(
                terminated(tag("Distance:"), space0),
                separated_list1(space1, u32),
            ),
        ),
        |(times, distances)| {
            times
                .iter()
                .zip(distances.iter())
                .map(|(&time, &record)| Race { time, record })
                .collect()
        },
    ).parse(input);

    result.unwrap().1
}

fn parse_single_race(input: &str) -> Input {
    parse_races(&input.replace(' ', ""))
}

#[derive(Debug)]
struct Race {
    time: u32,
    record: u32,
}

impl Race {
    /** Simulate a race holding the button for `button_time` */
    fn simulate(&self, button_time: u32) -> u32 {
        let speed = button_time;
        let remaining = self.time - button_time;

        speed * remaining
    }

    /** Get the number of ways to win by holding the button */
    fn simulate_all(&self) -> u32 {
        (0..self.time)
            .map(|x| self.simulate(x))
            .filter(|x| *x > self.record)
            .count() as u32
    }
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|x| x.simulate_all()).product()
}

fn problem2(input: &Input) -> u32 {
    input.first().unwrap().simulate_all()
}

#[cfg(test)]
mod test {
    use crate::{parse_races, parse_single_race, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse_races(input);
        let result = problem1(&input);
        assert_eq!(result, 288)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse_single_race(input);
        let result = problem2(&input);
        assert_eq!(result, 71503)
    }
}
