use nom::{
    character::complete::{i32, newline},
    multi::separated_list1,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<i32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, i32).parse(input);

    result.unwrap().1
}

fn problem1(x: &Input) -> u32 {
    todo!()
}

fn problem2(x: &Input) -> u32 {
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
        assert_eq!(result, 0);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
