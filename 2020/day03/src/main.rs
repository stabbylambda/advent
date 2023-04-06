use nom::{
    branch::alt,
    character::complete::{char, newline},
    multi::{many1, separated_list1},
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

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, many1(alt((char('#'), char('.')))))(input);

    result.unwrap().1
}

fn count_trees(input: &Input, dx: usize, dy: usize) -> u32 {
    let (mut x, mut y) = (0, 0);
    let mut trees = 0;

    let width = input[0].len();

    while y < input.len() {
        let current = input[y][x];
        trees += (current == '#') as u32;

        x = (x + dx) % width;
        y += dy;
    }

    trees
}

fn problem1(input: &Input) -> u32 {
    count_trees(input, 3, 1)
}

fn problem2(input: &Input) -> u32 {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 7)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 336)
    }
}
