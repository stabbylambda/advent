use std::collections::HashSet;

use nom::{branch::alt, character::complete::char, combinator::map, multi::many1, IResult, Parser};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Instruction>;

#[derive(Clone, Copy)]
enum Instruction {
    North,
    South,
    East,
    West,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(alt((
        map(char('^'), |_| Instruction::North),
        map(char('v'), |_| Instruction::South),
        map(char('<'), |_| Instruction::West),
        map(char('>'), |_| Instruction::East),
    ))).parse(input);

    result.unwrap().1
}

fn get_locations(input: &Input) -> HashSet<(i32, i32)> {
    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    let mut current = (0, 0);
    locations.insert((0, 0));
    for instruction in input {
        let (x, y) = current;
        current = match instruction {
            Instruction::North => (x, y + 1),
            Instruction::South => (x, y - 1),
            Instruction::East => (x + 1, y),
            Instruction::West => (x - 1, y),
        };

        locations.insert(current);
    }

    locations
}

fn problem1(input: &Input) -> usize {
    get_locations(input).len()
}

fn problem2(input: &Input) -> usize {
    let (santa, bot): (Vec<_>, Vec<_>) = input.chunks(2).map(|x| (x[0], x[1])).unzip();
    let santa = get_locations(&santa);
    let bot = get_locations(&bot);

    santa.union(&bot).count()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let tests = [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (input, expected) in tests {
            assert_eq!(problem1(&parse(input)), expected)
        }
    }

    #[test]
    fn second() {
        let tests = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];
        for (input, expected) in tests {
            assert_eq!(problem2(&parse(input)), expected)
        }
    }
}
