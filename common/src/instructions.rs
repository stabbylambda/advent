use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
pub fn instruction0<'a, R>(name: &'a str, r: R) -> impl Fn(&'a str) -> IResult<&'a str, R>
where
    R: Copy,
{
    move |s: &'a str| map(tag(name), |_| r)(s)
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
    move |s: &'a str| preceded(terminated(tag(name), char(' ')), map(x, f))(s)
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
        )(s)
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
    F: Copy + Fn(X1, Y1, Z1) -> R,
{
    move |s: &'a str| {
        preceded(
            terminated(tag(name), char(' ')),
            map(
                tuple((terminated(x, char(' ')), terminated(y, char(' ')), z)),
                |(x, y, z)| f(x, y, z),
            ),
        )(s)
    }
}
