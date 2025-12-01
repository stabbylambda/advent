use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input, 2503);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input, 2503);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Reindeer>;
#[derive(Debug)]
struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn distance_travelled(&self, time: u32) -> u32 {
        let t = self.duration + self.rest;

        let q = time / t;
        let r = time % t;

        self.speed * ((q * self.duration) + r.min(self.duration))
    }
}

fn parse(input: &str) -> Input {
    //Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    //Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            (
                alpha0,
                preceded(tag(" can fly "), nom_u32),
                preceded(tag(" km/s for "), nom_u32),
                delimited(
                    tag(" seconds, but then must rest for "),
                    nom_u32,
                    tag(" seconds."),
                ),
            ),
            |(_name, speed, duration, rest)| Reindeer {
                speed,
                duration,
                rest,
            },
        ),
    )
    .parse(input);

    result.unwrap().1
}

fn problem1(input: &Input, time: u32) -> u32 {
    input
        .iter()
        .map(|r| r.distance_travelled(time))
        .max()
        .unwrap()
}

fn problem2(input: &Input, time: u32) -> u32 {
    let scores = (1..=time)
        .flat_map(|n| {
            input
                .iter()
                .enumerate()
                .max_set_by_key(|(_idx, r)| r.distance_travelled(n))
        })
        .counts_by(|(idx, _r)| idx);

    scores
        .iter()
        .max_by_key(|x| x.1)
        .map(|(_idx, score)| *score as u32)
        .unwrap()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, 1000);
        assert_eq!(result, 1120)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input, 1000);
        assert_eq!(result, 689)
    }
}
