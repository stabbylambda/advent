use common::get_raw_input;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Node>;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    avail: u32,
}

impl Node {
    fn is_empty(&self) -> bool {
        self.used == 0
    }
}

fn parse(input: &str) -> Input {
    let name = preceded(
        tag("/dev/grid/node-"),
        separated_pair(preceded(tag("x"), u32), tag("-"), preceded(tag("y"), u32)),
    );

    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            tuple((
                terminated(name, multispace1),
                terminated(terminated(u32, tag("T")), multispace1),
                terminated(terminated(u32, tag("T")), multispace1),
                terminated(terminated(u32, tag("T")), multispace1),
                terminated(u32, tag("%")),
            )),
            |((x, y), size, used, avail, _used_percent)| Node {
                x,
                y,
                size,
                used,
                avail,
            },
        ),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input
        .iter()
        .filter(|a| !a.is_empty())
        .flat_map(|a| input.iter().filter_map(move |b| (a != b).then_some((a, b))))
        .filter(|(a, b)| a.used <= b.avail)
        .count()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 903)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
