use nom::{
    character::complete::{multispace0, newline, u32},
    multi::separated_list1,
    sequence::{preceded, tuple},
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

type Input = Vec<(u32, u32, u32)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        tuple((
            preceded(multispace0, u32),
            preceded(multispace0, u32),
            preceded(multispace0, u32),
        )),
    )(input);

    result.unwrap().1
}

fn is_triangle((a, b, c): &(u32, u32, u32)) -> bool {
    a + c > *b && a + b > *c && b + c > *a
}

fn problem1(input: &Input) -> usize {
    input.iter().filter(|x| is_triangle(x)).count()
}

fn problem2(input: &Input) -> usize {
    input
        .chunks(3)
        .flat_map(|v| {
            let a = v[0];
            let b = v[1];
            let c = v[2];
            vec![(a.0, b.0, c.0), (a.1, b.1, c.1), (a.2, b.2, c.2)]
        })
        .filter(is_triangle)
        .count()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }
}
