use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u32},
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
            separated_list1(tag(","), u32),
        ),
        |(springs, groups)| SpringRecord { springs, groups },
    )(s)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, parse_record)(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

#[derive(Debug)]
struct SpringRecord {
    springs: Vec<Spring>,
    groups: Vec<u32>,
}

impl SpringRecord {
    fn get_groups_with_count(&self) -> Vec<(u32, Spring)> {
        self.springs.iter().fold(vec![], |mut acc, spring| {
            let (last_count, last_spring) = acc.pop().unwrap_or((0, *spring));
            if last_spring == *spring {
                // update the current run
                acc.push((last_count + 1, last_spring))
            } else {
                // finish the last run and start a new one
                acc.push((last_count, last_spring));
                acc.push((1, *spring));
            }

            acc
        })
    }

    fn is_valid(&self) -> bool {
        let group_counts = self.get_groups_with_count();
        if group_counts
            .iter()
            .any(|(_count, spring)| spring == &Spring::Unknown)
        {
            false
        } else {
            group_counts
                .iter()
                .filter_map(|x| (x.1 == Spring::Broken).then_some(x.0))
                .collect::<Vec<u32>>()
                == self.groups
        }
    }
}

fn problem1(input: &Input) -> u32 {
    dbg!(input);
    todo!()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
