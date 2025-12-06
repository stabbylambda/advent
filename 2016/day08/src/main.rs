use ndarray::{s, Array2};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input, WIDTH, HEIGHT);
    println!("problem 1 answer: {answer}");
}

type Input = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Rectangle { width: usize, height: usize },
    RotateRow { index: usize, amount: usize },
    RotateColumn { index: usize, amount: usize },
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(
                preceded(tag("rect "), separated_pair(u32, char('x'), u32)),
                |(width, height)| Instruction::Rectangle {
                    width: width as usize,
                    height: height as usize,
                },
            ),
            map(
                preceded(tag("rotate row y="), separated_pair(u32, tag(" by "), u32)),
                |(index, amount)| Instruction::RotateRow {
                    index: index as usize,
                    amount: amount as usize,
                },
            ),
            map(
                preceded(
                    tag("rotate column x="),
                    separated_pair(u32, tag(" by "), u32),
                ),
                |(index, amount)| Instruction::RotateColumn {
                    index: index as usize,
                    amount: amount as usize,
                },
            ),
        )),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input, width: usize, height: usize) -> u32 {
    let mut arr: Array2<bool> = Array2::default((height, width));

    for instruction in input {
        match instruction {
            Instruction::Rectangle { width, height } => {
                // get the mutable slice in the upper left and fill with true
                arr.slice_mut(s![0..*height, 0..*width]).fill(true);
            }
            Instruction::RotateRow { index, amount } => {
                let shifted: Vec<bool> = arr
                    .row(*index)
                    .into_iter()
                    .copied()
                    .cycle()
                    .skip(width - *amount)
                    .take(width)
                    .collect();

                for (idx, v) in arr.row_mut(*index).indexed_iter_mut() {
                    *v = shifted[idx];
                }
            }
            Instruction::RotateColumn { index, amount } => {
                let shifted: Vec<bool> = arr
                    .column(*index)
                    .into_iter()
                    .copied()
                    .cycle()
                    .skip(height - *amount)
                    .take(height)
                    .collect();

                for (idx, v) in arr.column_mut(*index).indexed_iter_mut() {
                    *v = shifted[idx];
                }
            }
        }

        for x in arr.rows() {
            for y in x {
                print!(
                    "{}",
                    match y {
                        true => "█",
                        false => " ",
                    }
                );
            }

            println!();
        }
        println!();
    }

    arr.fold(0, |acc, x| acc + u32::from(*x))
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, 7, 3);
        assert_eq!(result, 6)
    }
}
