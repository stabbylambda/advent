use nom::{
    branch::alt,
    character::complete::{char, space1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};
fn main() {
    let lines = include_str!("../input.txt");

    let score = problem1(&lines);
    println!("problem 1 score: {score}");

    let score = problem2(&lines);
    println!("problem 2 score: {score}");
}

fn problem1(lines: &str) -> u32 {
    // using nom is overkill for this, but I figured there's gonna be a lot more parsing later so might as well
    // get some practice in
    fn get_pair(s: &str) -> (Hand, Hand) {
        fn get_hand(s: &str) -> IResult<&str, Hand> {
            alt((
                map(alt((char('A'), char('X'))), |_| Hand::Rock),
                map(alt((char('B'), char('Y'))), |_| Hand::Paper),
                map(alt((char('C'), char('Z'))), |_| Hand::Scissors),
            ))(s)
        }

        separated_pair(get_hand, space1, get_hand)(s).unwrap().1
    }

    lines
        .lines()
        .map(get_pair)
        .collect::<Vec<(Hand, Hand)>>()
        .iter()
        .map(|(o, s)| s.score(*o))
        .sum()
}

fn problem2(lines: &str) -> u32 {
    fn parse_hand(s: &str) -> IResult<&str, Hand> {
        alt((
            map(char('A'), |_| Hand::Rock),
            map(char('B'), |_| Hand::Paper),
            map(char('C'), |_| Hand::Scissors),
        ))(s)
    }

    fn parse_strategy(s: &str) -> IResult<&str, Strategy> {
        alt((
            map(char('X'), |_| Strategy::Lose),
            map(char('Y'), |_| Strategy::Draw),
            map(char('Z'), |_| Strategy::Win),
        ))(s)
    }

    fn get_hands(s: &str) -> (Hand, Strategy) {
        separated_pair(parse_hand, space1, parse_strategy)(s)
            .unwrap()
            .1
    }

    lines
        .lines()
        .map(get_hands)
        .collect::<Vec<(Hand, Strategy)>>()
        .iter()
        .map(|(o, s)| s.get_hand(*o).score(*o))
        .sum()
}

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Strategy {
    Win = 1,
    Lose = -1,
    Draw = 0,
}

impl Strategy {
    fn get_hand(&self, other: Hand) -> Hand {
        // there's definitely a way to mod math this one too, but I am laaaazy
        match (other, self) {
            (Hand::Rock, Strategy::Win) => Hand::Paper,
            (Hand::Rock, Strategy::Lose) => Hand::Scissors,
            (Hand::Rock, Strategy::Draw) => Hand::Rock,
            (Hand::Paper, Strategy::Win) => Hand::Scissors,
            (Hand::Paper, Strategy::Lose) => Hand::Rock,
            (Hand::Paper, Strategy::Draw) => Hand::Paper,
            (Hand::Scissors, Strategy::Win) => Hand::Rock,
            (Hand::Scissors, Strategy::Lose) => Hand::Paper,
            (Hand::Scissors, Strategy::Draw) => Hand::Scissors,
        }
    }
}

impl Hand {
    fn score(&self, other: Hand) -> u32 {
        let s = *self as u32;
        let o = other as u32;

        let number_score = if o == (s + 1) % 3 {
            0
        } else if o == s {
            3
        } else {
            6
        };

        number_score + s + 1
    }
}

#[cfg(test)]
mod test {

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let lines = include_str!("../test.txt");
        let score = problem1(&lines);
        assert_eq!(score, 15)
    }

    #[test]
    fn second() {
        let lines = include_str!("../test.txt");
        let score = problem2(&lines);
        assert_eq!(score, 12)
    }
}
