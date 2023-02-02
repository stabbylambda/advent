use common::get_raw_input;
use nom::{
    character::complete::{char, newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<(u32, u32)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_pair(u32, char('-'), u32))(input);

    let mut input = result.unwrap().1;
    input.sort();
    input
}

fn get_contiguous_ranges(ranges: &[(u32, u32)]) -> Vec<(u32, u32)> {
    ranges.iter().fold(vec![], |mut acc, &r @ (ra, rb)| {
        // push the first item on the stack
        match acc.pop() {
            Some(previous @ (pa, pb)) => {
                // ranges don't always fully overlap, so we need to make sure contiguous ranges are accounted for
                if ra <= pb.saturating_add(1) {
                    acc.push((pa, rb.max(pb)))
                } else {
                    acc.push(previous);
                    acc.push(r);
                }
            }
            None => acc.push(r),
        };

        acc
    })
}

fn problem1(input: &Input) -> u32 {
    let disallowed = get_contiguous_ranges(&input[..]);
    // find the first one that doesn't start with 0 and find the first one before the range
    disallowed
        .iter()
        .find_map(|(start, _end)| (*start != 0).then(|| start - 1))
        .unwrap()
}

fn problem2(input: &Input) -> u32 {
    let disallowed = get_contiguous_ranges(&input[..]);
    // find all the IPs between the ranges. this assumes that input starts with 0 and ends with u32::MAX, which it does
    disallowed
        .windows(2)
        .map(|w| {
            let (_aa, ab) = w[0];
            let (ba, _bb) = w[1];

            ba - ab - 1
        })
        .sum()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 3)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 1)
    }
}
