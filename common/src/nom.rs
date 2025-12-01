use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, newline, u32},
    combinator::{map, map_opt},
    error::ParseError,
    multi::{many1, many_till, separated_list1},
    sequence::separated_pair,
    IResult, Input, Parser,
};

use crate::grid::{Coord, Grid};

pub fn parse_grid<I, O, E: ParseError<I>, F: Parser<I, Output = O, Error = E>>(
    f: F,
) -> impl Parser<I, Output = Grid<O>, Error = E>
where
    I: Clone + Input,
    O: Copy,
    <I as nom::Input>::Item: nom::AsChar,
{
    map(separated_list1(newline, many1(f)), Grid::new)
}

pub fn drop_till<I, O, E: ParseError<I>, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
where
    I: Clone + Input,
    F: Parser<I, Output = O, Error = E>,
{
    map(many_till(take(1u8), parser), |(_, matched)| matched)
}

pub fn single_digit(s: &str) -> IResult<&str, u32> {
    map_opt(anychar, |c| c.to_digit(10)).parse(s)
}

pub fn coord(s: &str) -> IResult<&str, Coord> {
    map(separated_pair(u32, tag(","), u32), |(x, y)| {
        (x as usize, y as usize)
    })
    .parse(s)
}

pub fn usize(s: &str) -> IResult<&str, usize> {
    map(u32, |x| x as usize).parse(s)
}
