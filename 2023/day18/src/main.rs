use common::map::Direction;
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

type Input<'a> = Vec<(Direction, u32, &'a str)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        tuple((
            terminated(
                alt((
                    map(char('U'), |_| Direction::North),
                    map(char('D'), |_| Direction::South),
                    map(char('L'), |_| Direction::West),
                    map(char('R'), |_| Direction::East),
                )),
                tag(" "),
            ),
            terminated(u32, tag(" ")),
            delimited(tag("(#"), alphanumeric1, tag(")")),
        )),
    )(input);

    result.unwrap().1
}

fn generate_points<'a>(directions: &[(Direction, u32, &'a str)]) -> (i64, Vec<(i64, i64)>) {
    let mut points: Vec<(i64, i64)> = vec![];
    let (mut x, mut y) = (0, 0);
    let mut length = 0i64;
    for (direction, d, color) in directions {
        length += *d as i64;
        for _n in 0..*d {
            (x, y) = match direction {
                Direction::North => (x, y - 1),
                Direction::South => (x, y + 1),
                Direction::West => (x - 1, y),
                Direction::East => (x + 1, y),
            };
        }
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

fn problem1(input: &Input) -> i64 {
    let (length, mut vertices) = generate_points(input);

    let area = shoelace(&vertices).abs();
    let area = if area.is_negative() {
        vertices.reverse();
        shoelace(&vertices)
    } else {
        area
    };

    let inside = picks_theorem(area, length);

    inside + length
}

fn problem2(input: &Input) -> u32 {
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
        assert_eq!(result, 62)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
