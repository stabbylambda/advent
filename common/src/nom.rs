use std::ops::RangeFrom;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, newline, u32},
    combinator::{map, map_opt},
    error::ParseError,
    multi::{many1, many_till, separated_list1},
    sequence::separated_pair,
    IResult, InputIter, InputLength, InputTake, Parser, Slice,
};

use crate::grid::{Coord, Grid};

pub fn parse_grid<I, O, E: ParseError<I>, F: Parser<I, O, E>>(
    f: F,
) -> impl FnMut(I) -> IResult<I, Grid<O>, E>
where
    I: Clone + InputIter + InputLength + Slice<RangeFrom<usize>>,
    O: Copy,
    <I as nom::InputIter>::Item: nom::AsChar,
{
    map(separated_list1(newline, many1(f)), Grid::new)
}

pub fn drop_till<I, O, E: ParseError<I>, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: Clone + InputLength + InputTake + InputIter,
    F: Parser<I, O, E>,
{
    map(many_till(take(1u8), parser), |(_, matched)| matched)
}

pub fn single_digit(s: &str) -> IResult<&str, u32> {
    map_opt(anychar, |c| c.to_digit(10))(s)
}

pub fn coord(s: &str) -> IResult<&str, Coord> {
    map(separated_pair(u32, tag(","), u32), |(x, y)| {
        (x as usize, y as usize)
    })(s)
}

pub fn usize(s: &str) -> IResult<&str, usize> {
    map(u32, |x| x as usize)(s)
}
