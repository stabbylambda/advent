use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u32 as nom_u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::{Input, Instruction, Space};

pub(crate) fn parse_grid(input: &str) -> IResult<&str, Array2<Space>> {
    map(
        separated_list1(
            newline,
            many1(alt((
                map(char(' '), |_| Space::Void),
                map(char('.'), |_| Space::Empty),
                map(char('#'), |_| Space::Wall),
            ))),
        ),
        to_array,
    )(input)
}

pub(crate) fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        map(nom_u32, Instruction::Walk),
        map(char('L'), |_| Instruction::TurnLeft),
        map(char('R'), |_| Instruction::TurnRight),
    )))(input)
}

pub(crate) fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_pair(parse_grid, tag("\n\n"), parse_instructions)(input);

    result.unwrap().1
}

fn to_array(grid: Vec<Vec<Space>>) -> Array2<Space> {
    let height = grid.len();
    let width = grid.iter().map(|x| x.len()).max().unwrap();

    let mut new_grid = Array2::from_elem((height, width), Space::Void);

    // pack the vec vec into an ndarray
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_grid[[y, x]] = *cell;
        }
    }

    new_grid
}
