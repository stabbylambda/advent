use nom::{
    branch::alt,
    character::{
        complete::{char, newline, space0, space1, u64},
        one_of,
    },
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input1 = parse_part1(input);
    let input2 = parse_part2(input);

    let score = problem1(&input1);
    println!("problem 1 score: {score}");

    let score = problem2(&input2);
    println!("problem 2 score: {score}");
}

type Input = Vec<Column>;

#[derive(Debug)]
struct Column {
    numbers: Vec<u64>,
    operation: char,
}

impl Column {
    fn calculate(&self) -> u64 {
        self.numbers
            .iter()
            .cloned()
            .reduce(|acc, x| match self.operation {
                '+' => acc + x,
                '*' => acc * x,
                _ => unreachable!(),
            })
            .unwrap()
    }
}

fn parse_part1(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            separated_list1(newline, many1(delimited(space0, u64, space0))),
            newline,
            separated_list1(space0, alt((char('+'), char('*')))),
        ),
        |(rows, operations)| {
            operations
                .iter()
                .enumerate()
                .map(|(idx, &operation)| Column {
                    numbers: rows.iter().map(|r| r[idx]).collect(),
                    operation,
                })
                .collect()
        },
    )
    .parse(input);

    result.unwrap().1
}

fn parse_part2(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            separated_list1(newline, many1(one_of(" 0123456789"))),
            newline,
            separated_list1(space1, alt((char('+'), char('*')))),
        ),
        |(grid, operations)| {
            let height = grid.len();
            let width = grid[0].len();

            let mut numbers = vec![];
            let mut group = vec![];

            // read the grid from right to left
            for c in (0..width).rev() {
                let mut num = 0;
                // go top to bottom in each column
                (0..height).for_each(|r| {
                    let c = grid[r][c];
                    // if there's a digit, add it to the correct place
                    if let Some(c) = c.to_digit(10) {
                        let c: u64 = c.into();
                        num = num * 10 + c;
                    }
                });

                // if we saw a whole column of spaces, we know the next column is the first number
                // of the next set
                if num == 0 {
                    numbers.push(group.clone());
                    group = vec![];
                } else {
                    group.push(num);
                }
            }

            // push the last set
            numbers.push(group);

            // zip it with the operations
            operations
                .iter()
                .rev()
                .zip(numbers)
                .map(|(op, numbers)| Column {
                    numbers,
                    operation: *op,
                })
                .collect()
        },
    )
    .parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> u64 {
    x.iter().map(|c| c.calculate()).sum()
}

fn problem2(x: &Input) -> u64 {
    x.iter().map(|c| c.calculate()).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse_part1, parse_part2, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse_part1(input);
        let result = problem1(&input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse_part2(input);
        let result = problem2(&input);
        assert_eq!(result, 3263827)
    }
}
