use std::collections::VecDeque;

use cached::proc_macro::cached;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
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

type Input = Vec<SpringRecord>;

fn parse_record(s: &str) -> IResult<&str, SpringRecord> {
    map(
        separated_pair(
            many1(alt((
                map(char('.'), |_| Spring::Operational),
                map(char('#'), |_| Spring::Broken),
                map(char('?'), |_| Spring::Unknown),
            ))),
            tag(" "),
            separated_list1(tag(","), u64),
        ),
        |(springs, groups)| SpringRecord::new(VecDeque::from(springs), VecDeque::from(groups)),
    )(s)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, parse_record)(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct SpringRecord {
    springs: VecDeque<Spring>,
    groups: VecDeque<u64>,
}

impl SpringRecord {
    fn new(springs: VecDeque<Spring>, groups: VecDeque<u64>) -> Self {
        Self { springs, groups }
    }

    fn multiply(&self, count: u64) -> Self {
        let springs: VecDeque<Spring> = (0..count)
            .flat_map(|x| {
                let mut s = self.springs.clone();
                if x < count - 1 {
                    s.push_back(Spring::Unknown);
                }
                s
            })
            .collect();

        let groups: VecDeque<u64> = (0..count).flat_map(|_| self.groups.clone()).collect();

        Self { springs, groups }
    }

    fn remaining_count(&self) -> u64 {
        self.groups.iter().sum()
    }

    fn possible_broken(&self) -> u64 {
        self.springs
            .iter()
            .filter(|&&x| x == Spring::Broken || x == Spring::Unknown)
            .count() as u64
    }

    fn consume_spring(&self) -> Self {
        let mut new = self.clone();
        new.springs.pop_front();
        new
    }

    fn consume_spring_and_group(&self) -> Self {
        let mut new = self.clone();
        new.springs.pop_front();
        new.groups.pop_front();
        new
    }
}

// Memoize the function. I hate dynamic programming problems.
#[cached]
fn count_solutions(sr: SpringRecord, current_run: Option<u64>) -> u64 {
    // If there are no springs left, then we're done
    if sr.springs.is_empty() {
        match (sr.groups.len(), current_run, sr.groups.front()) {
            // no groups left, no run
            (0, None, _) => return 1,
            // one group left, same size as current run
            (1, Some(run), Some(&last_group)) if run == last_group => return 1,
            // otherwise no solutions
            _ => return 0,
        };
    }

    // if we have a run going, but there are no groups left, then this isn't solveable
    if let (Some(_), None) = (current_run, sr.groups.front()) {
        return 0;
    }

    // we can't solve it if we don't have enough to even match the rest of the checksums
    let possible_current_run = current_run.unwrap_or(0);
    if sr.possible_broken() + possible_current_run < sr.remaining_count() {
        return 0;
    }

    let details = (sr.springs[0], current_run, sr.groups.front().cloned());

    let mut possible = 0;

    // if we hit an operational spring, but our run doesn't match the expected group, this isn't a solution
    if let (Spring::Operational, Some(current_run), Some(next_group)) = details {
        if current_run != next_group {
            return 0;
        }
    }

    // If we hit operational spring with a run going, consume the spring and the group and clear the run
    if let (Spring::Operational, Some(_), _) = details {
        possible += count_solutions(sr.consume_spring_and_group(), None);
    }

    // If we hit unknown spring, and the run matches the group, act as if this is operational
    if let (Spring::Unknown, Some(current_run), Some(next_group)) = details {
        if current_run == next_group {
            possible += count_solutions(sr.consume_spring_and_group(), None);
        }
    }

    // If we hit broken or unknown, consume the spring and keep going down the run or start a new one
    if let (Spring::Broken | Spring::Unknown, _, _) = details {
        let run = current_run.unwrap_or(0);
        possible += count_solutions(sr.consume_spring(), Some(run + 1));
    }

    // if we hit operational or unknown with no run going, consume the spring and wipe the run
    if let (Spring::Unknown | Spring::Operational, None, _) = details {
        possible += count_solutions(sr.consume_spring(), None);
    }

    possible
}

fn problem1(input: &Input) -> u64 {
    input.iter().map(|x| count_solutions(x.clone(), None)).sum()
}

fn problem2(input: &Input) -> u64 {
    input
        .iter()
        .map(|sr| count_solutions(sr.multiply(5), None))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 21)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 525152)
    }
}
