use common::grid::CardinalDirection;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
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

type Input<'a> = Vec<(CardinalDirection, u32, &'a str)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        tuple((
            terminated(
                alt((
                    map(char('U'), |_| CardinalDirection::North),
                    map(char('D'), |_| CardinalDirection::South),
                    map(char('L'), |_| CardinalDirection::West),
                    map(char('R'), |_| CardinalDirection::East),
                )),
                tag(" "),
            ),
            terminated(u32, tag(" ")),
            delimited(tag("(#"), alphanumeric1, tag(")")),
        )),
    )(input);

    result.unwrap().1
}

fn generate_points(directions: &[(CardinalDirection, u32)]) -> (i64, Vec<(i64, i64)>) {
    let mut points: Vec<(i64, i64)> = vec![];
    let (mut x, mut y) = (0i64, 0i64);
    let mut length = 0i64;
    for (direction, d) in directions {
        let d = *d as i64;

        // keep track of the lengths
        length += d;

        (x, y) = match direction {
            CardinalDirection::North => (x, y - d),
            CardinalDirection::South => (x, y + d),
            CardinalDirection::West => (x - d, y),
            CardinalDirection::East => (x + d, y),
        };

        // push the vertex only
        points.push((x, y));
    }

    (length, points)
}

/** Do the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula#Shoelace_formula) */
fn shoelace(vertices: &[(i64, i64)]) -> i64 {
    let area: i64 = vertices
        .windows(2)
        .map(|w| {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];

            (y1 + y2) * (x2 - x1)
        })
        .sum();

    area / 2
}

/** [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) */
fn picks_theorem(area: i64, length: i64) -> i64 {
    area - length / 2 + 1
}

fn calculate_area(directions: &[(CardinalDirection, u32)]) -> i64 {
    let (length, vertices) = generate_points(directions);
    let area = shoelace(&vertices);
    let inside = picks_theorem(area.abs(), length);

    inside + length
}

fn problem1(input: &Input) -> i64 {
    let directions: Vec<(CardinalDirection, u32)> =
        input.iter().map(|(dir, len, _)| (*dir, *len)).collect();

    calculate_area(&directions)
}

fn problem2(input: &Input) -> i64 {
    let v: Vec<(CardinalDirection, u32)> = input
        .iter()
        .map(|(_, _, hex)| {
            let dir = match hex.chars().nth(5) {
                Some('0') => CardinalDirection::East,
                Some('1') => CardinalDirection::South,
                Some('2') => CardinalDirection::West,
                Some('3') => CardinalDirection::North,
                _ => unreachable!(),
            };

            let length = hex
                .get(0..5)
                .and_then(|s| u32::from_str_radix(s, 16).ok())
                .unwrap();

            (dir, length)
        })
        .collect();

    calculate_area(&v)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 62)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 952408144115)
    }
}
