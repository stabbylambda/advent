use nom::{branch::alt, character::complete::char, combinator::map, multi::many1, IResult, Parser};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Row;

#[derive(Clone)]
struct Row {
    tiles: Vec<u8>,
}

impl Row {
    fn generate_next(&self) -> Row {
        let mut v = self.tiles.clone();

        // make sure the walls are safe
        v.insert(0, 0);
        v.push(0);

        let tiles = v.windows(3).map(|v| u8::from(v[0] != v[2])).collect();

        Row { tiles }
    }

    fn count_safe(&self) -> usize {
        self.tiles.iter().filter(|&&t| t == 0).count()
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        many1(alt((map(char('.'), |_| 0), map(char('^'), |_| 1)))),
        |v| Row { tiles: v },
    ).parse(input);

    result.unwrap().1
}

fn problem(input: &Input, row_count: usize) -> usize {
    let mut row: Row = input.clone();
    let mut safe_count = input.count_safe();

    for _ in 1..row_count {
        row = row.generate_next();
        safe_count += row.count_safe();
    }

    safe_count
}

fn problem1(input: &Input) -> usize {
    problem(input, 40)
}

fn problem2(input: &Input) -> usize {
    problem(input, 400_000)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 185)
    }
}
