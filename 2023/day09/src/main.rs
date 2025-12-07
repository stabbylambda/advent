use common::{answer, read_input};
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Vec<i64>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_list1(tag(" "), i64)).parse(input);

    result.unwrap().1
}

fn get_to_zeros(v: &[i64]) -> Vec<Vec<i64>> {
    let mut rows = vec![v.to_vec()];
    while let Some(last_row) = rows.last() {
        // create a new row with the deltas between all of the items
        let new_row = last_row
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect::<Vec<i64>>();

        let is_zero_row = new_row.iter().all(|x| *x == 0);

        // if we got to the zero row, bail
        if is_zero_row {
            break;
        }
        rows.push(new_row);
    }

    // reverse it so we always start from the bottom
    rows.reverse();
    rows
}

fn problem1(input: &Input) -> i64 {
    input
        .iter()
        .map(|x| {
            // Accumulate up the chain again, adding the last item on each row with the new delta below it
            get_to_zeros(x)
                .iter()
                .fold(0, |acc, x| acc + x.last().unwrap())
        })
        .sum()
}

fn problem2(input: &Input) -> i64 {
    input
        .iter()
        .map(|x| {
            // Accumulate up the chain again, subtracting the new delta from the first item in each row
            get_to_zeros(x)
                .iter()
                .fold(0, |acc, x| x.first().unwrap() - acc)
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
        assert_eq!(result, 114)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2)
    }
}
