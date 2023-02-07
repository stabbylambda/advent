use nom::{
    character::complete::{char, u32 as nom_u32},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let lines = include_str!("../input.txt");
    let assignments = parse_assignments(lines);

    let score = problem1(&assignments);
    println!("problem 1 score: {score}");

    let score = problem2(&assignments);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
struct Range(u32, u32);
impl Range {
    fn fully_contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    fn partially_contains(&self, other: &Range) -> bool {
        let other_start_in_range = self.0 <= other.0 && other.0 <= self.1;
        let other_end_in_range = self.0 <= other.1 && other.1 <= self.1;

        other_start_in_range || other_end_in_range
    }

    fn parse(s: &str) -> IResult<&str, Range> {
        map(
            separated_pair(nom_u32, char('-'), nom_u32),
            |(start, end)| Range(start, end),
        )(s)
    }
}
#[derive(Debug)]
struct Assignment {
    first: Range,
    second: Range,
}
impl Assignment {
    fn is_full_overlap(&self) -> bool {
        self.first.fully_contains(&self.second) || self.second.fully_contains(&self.first)
    }

    fn is_any_overlap(&self) -> bool {
        self.first.partially_contains(&self.second) || self.second.partially_contains(&self.first)
    }

    fn parse(s: &str) -> IResult<&str, Assignment> {
        map(
            separated_pair(Range::parse, char(','), Range::parse),
            |(first, second)| Assignment { first, second },
        )(s)
    }
}

fn parse_assignments(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(|s| Assignment::parse(s).unwrap().1)
        .collect()
}

fn problem1(assignments: &[Assignment]) -> u32 {
    assignments.iter().filter(|x| x.is_full_overlap()).count() as u32
}

fn problem2(assignments: &[Assignment]) -> u32 {
    assignments.iter().filter(|x| x.is_any_overlap()).count() as u32
}

#[cfg(test)]
mod test {

    use crate::{parse_assignments, problem1, problem2};
    #[test]
    fn first() {
        let lines = include_str!("../test.txt");
        let assignments = parse_assignments(lines);
        let result = problem1(&assignments);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let lines = include_str!("../test.txt");
        let assignments = parse_assignments(lines);
        let result = problem2(&assignments);
        assert_eq!(result, 4)
    }
}
