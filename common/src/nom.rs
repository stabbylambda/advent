use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, u32},
    combinator::{map, map_opt},
    error::ParseError,
    multi::many_till,
    sequence::separated_pair,
    IResult, InputIter, InputLength, InputTake, Parser,
};

use crate::map::Coord;

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
