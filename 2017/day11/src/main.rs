use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, IResult, Parser};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input = Vec<(i32, i32)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag(","),
        alt((
            map(tag("ne"), |_| (1, -1)),
            map(tag("nw"), |_| (0, 1)),
            map(tag("se"), |_| (0, -1)),
            map(tag("sw"), |_| (-1, 1)),
            map(tag("n"), |_| (1, 0)),
            map(tag("s"), |_| (-1, 0)),
        )),
    ).parse(input);

    result.unwrap().1
}

fn problem(input: &Input) -> (i32, i32) {
    /* This is awesome. The hex movement is obviously vector addition, but once you get that, you're
    in hex coordinates with weird rows. I bashed my head against a "normal" hex system for a bit and then found
    https://stackoverflow.com/a/5085274  where the axes are rotated so you can do normal distances between hexes.
    */
    let (mut x, mut y) = (0, 0);
    let mut max_result = 0;
    let mut result = 0;

    for (dx, dy) in input {
        x += dx;
        y += dy;

        result = if x.signum() == y.signum() {
            (x + y).abs()
        } else {
            x.abs().max(y.abs())
        };

        max_result = max_result.max(result);
    }

    (result, max_result)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let cases = [
            ("ne,ne,ne", 3, 3),
            ("ne,ne,sw,sw", 0, 2),
            ("ne,ne,s,s", 2, 2),
            ("se,sw,se,sw,sw", 3, 3),
        ];
        for (input, expected_distance, expected_max) in cases {
            let input = parse(input);
            let (distance, max) = problem(&input);
            assert_eq!(distance, expected_distance);
            assert_eq!(max, expected_max);
        }
    }
}
