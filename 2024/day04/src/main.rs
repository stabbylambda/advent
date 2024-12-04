use common::map::{Map, MapSquare};
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Map<char>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            many1(alt((char('X'), char('M'), char('A'), char('S')))),
        ),
        Map::new,
    )(input);
    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input.into_iter().map(|x| spells(&x, "XMAS", None)).sum()
}

fn spells(x: &MapSquare<'_, char>, s: &str, dir: Option<usize>) -> usize {
    match s.chars().next() {
        Some(c) if &c != x.data => return 0,
        Some('S') if x.data == &'S' => {
            return 1;
        }
        _ => {}
    };

    let remaining = &s[1..];
    let n = x.all_neighbors();
    let v = vec![
        n.north_west,
        n.north,
        n.north_east,
        n.west,
        n.east,
        n.south_west,
        n.south,
        n.south_east,
    ];

    v.iter()
        .enumerate()
        .filter(|(d, _)| match dir {
            None => true,
            Some(x) => x == *d,
        })
        .filter_map(|(d, x)| x.map(|x| (d, x)))
        .map(|(d, n)| spells(&n, remaining, Some(d)))
        .sum()
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
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 18)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
