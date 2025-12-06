use cached::{Cached, UnboundCache};

use common::nom::usize;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
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
            separated_list1(tag(","), usize),
        ),
        |(springs, groups)| SpringRecord::new(springs, groups),
    ).parse(s)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, parse_record).parse(input);

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
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl SpringRecord {
    fn new(springs: Vec<Spring>, groups: Vec<usize>) -> Self {
        Self { springs, groups }
    }

    fn multiply(&self, count: u64) -> Self {
        let springs: Vec<Spring> = (0..count)
            .flat_map(|x| {
                let mut s = self.springs.clone();
                if x < count - 1 {
                    s.push(Spring::Unknown);
                }
                s
            })
            .collect();

        let groups: Vec<usize> = (0..count).flat_map(|_| self.groups.clone()).collect();

        Self { springs, groups }
    }
}

type SolutionCache = UnboundCache<(usize, usize, Option<usize>), usize>;
fn count_solutions(
    cache: &mut SolutionCache,
    springs: &[Spring],
    groups: &[usize],
    current_run: Option<usize>,
) -> usize {
    use Spring::*;
    // hit the cache first to see if we have a solution already
    let key = (springs.len(), groups.len(), current_run);
    if let Some(x) = cache.cache_get(&key) {
        return *x;
    }

    // these are all the terminating conditions
    match (springs.len(), groups.len(), current_run) {
        // no springs, no groups left, no run
        (0, 0, None) => return 1,
        // no springs, one group left, same size as current run
        (0, 1, Some(run)) if run == groups[0] => return 1,
        // no springs, anything else, no solution
        (0, _, _) => return 0,
        // springs, no groups left, in a run
        (_, 0, Some(_)) => return 0,
        _ => (),
    };

    // we know we have at least one spring, so destructure it out
    let [spring, springs @ ..] = springs else {
        unreachable!()
    };

    let possible = match (spring, current_run, groups.first()) {
        // if we hit an operational spring, but our run doesn't match the expected group, this isn't a solution
        (Operational, Some(current_run), Some(&next_group)) if current_run != next_group => 0,

        // If we hit operational spring with a run going, consume the spring and the group and clear the run
        (Operational, Some(_), _) => count_solutions(cache, springs, &groups[1..], None),

        // if we hit operational with no run going, consume the spring, keep going
        (Operational, None, _) => count_solutions(cache, springs, groups, None),

        // If we hit broken, consume the spring and keep going down the run or start a new one
        (Broken, Some(run), _) => count_solutions(cache, springs, groups, Some(run + 1)),
        (Broken, None, _) => count_solutions(cache, springs, groups, Some(1)),

        // If we hit unknown spring, and the run matches the group, we need to pursue two paths
        (Unknown, Some(current_run), Some(&next_group)) if current_run == next_group => {
            let new_run = count_solutions(cache, springs, &groups[1..], None);
            let current_run = count_solutions(cache, springs, groups, Some(current_run + 1));

            new_run + current_run
        }
        (Unknown, Some(current_run), _) => {
            count_solutions(cache, springs, groups, Some(current_run + 1))
        }

        // if we hit an unknown with no run going, we need to consider both universes
        (Unknown, None, _) => {
            let broken = count_solutions(cache, springs, groups, Some(1));
            let fixed = count_solutions(cache, springs, groups, None);

            broken + fixed
        }
    };

    cache.cache_set(key, possible);

    possible
}

fn problem1(input: &Input) -> usize {
    input
        .iter()
        .map(|x| {
            let sr = x.clone();
            count_solutions(&mut UnboundCache::new(), &sr.springs, &sr.groups, None)
        })
        .sum()
}

fn problem2(input: &Input) -> usize {
    input
        .iter()
        .map(|sr| {
            let sr = sr.multiply(5);
            count_solutions(&mut UnboundCache::new(), &sr.springs, &sr.groups, None)
        })
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
