use nom::{character::complete::anychar, combinator::map_opt, IResult};

pub fn single_digit(s: &str) -> IResult<&str, u32> {
    map_opt(anychar, |c| c.to_digit(10))(s)
}
