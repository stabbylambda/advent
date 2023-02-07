use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, u32},
    combinator::map,
    multi::{count, many1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
enum Section {
    Text(String),
    Repeat { chars: String, count: usize },
}
impl Section {
    fn len(&self, expand: bool) -> usize {
        match self {
            Section::Text(a) => a.len(),
            Section::Repeat { chars, count } => {
                let inner = if expand {
                    let input = parse(chars);
                    let parsed = input.iter().fold(0, |acc, x| acc + x.len(true));
                    parsed
                } else {
                    chars.len()
                };

                inner * count
            }
        }
    }
}
type Input = Vec<Section>;

fn repeat(input: &str) -> IResult<&str, Section> {
    let (input, (chars, repeat)) =
        delimited(tag("("), separated_pair(u32, tag("x"), u32), tag(")"))(input)?;

    let (input, v) = count(anychar, chars as usize)(input)?;

    Ok((
        input,
        Section::Repeat {
            chars: v.into_iter().collect(),
            count: repeat as usize,
        },
    ))
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(alt((
        map(alphanumeric1, |x: &str| Section::Text(x.to_string())),
        repeat,
    )))(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input.iter().fold(0, |acc, x| acc + x.len(false))
}

fn problem2(input: &Input) -> usize {
    input.iter().fold(0, |acc, x| acc + x.len(true))
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let tests = [
            ("ADVENT", 6),
            ("A(1x5)BC", 7),
            ("(3x3)XYZ", 9),
            ("A(2x2)BCD(2x2)EFG", 11),
            ("(6x1)(1x3)A", 6),
            ("X(8x2)(3x3)ABCY", 18),
        ];
        for (input, expected) in tests {
            let input = parse(input);
            let result = problem1(&input);
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn second() {
        let tests = [
            ("(3x3)XYZ", 9),
            ("X(8x2)(3x3)ABCY", 20),
            ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920),
            (
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                445,
            ),
        ];
        for (input, expected) in tests {
            let input = parse(input);
            let result = problem2(&input);
            assert_eq!(result, expected)
        }
    }
}
