use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Nanobot>;

struct Nanobot {
    position: (i64, i64, i64),
    radius: u64,
}

impl Nanobot {
    fn distance(&self, other: &Nanobot) -> u64 {
        let (sx, sy, sz) = self.position;
        let (ox, oy, oz) = other.position;

        sx.abs_diff(ox) + sy.abs_diff(oy) + sz.abs_diff(oz)
    }

    fn in_range(&self, other: &Nanobot) -> bool {
        self.distance(other) <= self.radius
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                delimited(
                    tag("pos=<"),
                    tuple((terminated(i64, tag(",")), terminated(i64, tag(",")), i64)),
                    tag(">"),
                ),
                tag(", "),
                preceded(tag("r="), u64),
            ),
            |(position, radius)| Nanobot { position, radius },
        ),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let strongest = input.iter().max_by_key(|x| x.radius).unwrap();

    input
        .iter()
        .filter(|other| strongest.in_range(other))
        .count()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
