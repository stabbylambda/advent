use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, i64},
    combinator::map,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

use super::registers::{Register, Value};
pub fn value(s: &str) -> IResult<&str, Value> {
    alt((map(i64, Value::Literal), map(anychar, Value::Register)))
    .parse(s)
}

pub fn register(s: &str) -> IResult<&str, Register> {
    anychar(s)
}

pub fn instruction0<'a, R>(name: &'a str, r: R) -> impl Fn(&'a str) -> IResult<&'a str, R>
where
    R: Copy,
{
    move |s: &'a str| map(tag(name), |_| r).parse(s)
}

pub fn instruction1<'a, X, X1, F, R>(
    name: &'a str,
    x: X,
    f: F,
) -> impl Fn(&'a str) -> IResult<&'a str, R>
where
    X: Copy + Fn(&'a str) -> IResult<&'a str, X1>,
    F: Copy + Fn(X1) -> R,
{
    move |s: &'a str| preceded(terminated(tag(name), char(' ')), map(x, f)).parse(s)
}

pub fn instruction2<'a, X, X1, Y, Y1, F, R>(
    name: &'a str,
    x: X,
    y: Y,
    f: F,
) -> impl Fn(&'a str) -> IResult<&'a str, R>
where
    X: Copy + Fn(&'a str) -> IResult<&'a str, X1>,
    Y: Copy + Fn(&'a str) -> IResult<&'a str, Y1>,
    F: Copy + Fn(X1, Y1) -> R,
{
    move |s: &'a str| {
        preceded(
            terminated(tag(name), char(' ')),
            map(separated_pair(x, char(' '), y), |(x, y)| f(x, y)),
        )
        .parse(s)
    }
}

pub fn instruction3<'a, X, X1, Y, Y1, Z, Z1, F, R>(
    name: &'a str,
    x: X,
    y: Y,
    z: Z,
    f: F,
) -> impl Fn(&'a str) -> IResult<&'a str, R>
where
    X: Copy + Fn(&'a str) -> IResult<&'a str, X1>,
    Y: Copy + Fn(&'a str) -> IResult<&'a str, Y1>,
    Z: Copy + Fn(&'a str) -> IResult<&'a str, Z1>,
    F: Fn(X1, Y1, Z1) -> R,
{
    move |s: &'a str| {
        preceded(
            terminated(tag(name), char(' ')),
            map(
                (terminated(x, char(' ')), terminated(y, char(' ')), z),
                |(x, y, z)| f(x, y, z),
            ),
        )
        .parse(s)
    }
}
