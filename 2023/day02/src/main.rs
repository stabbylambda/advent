use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Game>;

struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn new(colors: Vec<Color>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        // this is kinda gross, but there really wasn't a better way because RGB isn't always in the right order
        for c in colors {
            match c {
                Color::Red(x) => red = x,
                Color::Green(x) => green = x,
                Color::Blue(x) => blue = x,
            }
        }

        Self { red, green, blue }
    }
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.hands
            .iter()
            .all(|h| h.red <= 12 && h.green <= 13 && h.blue <= 14)
    }

    fn power(&self) -> u32 {
        let red = self.hands.iter().map(|x| x.red).max().unwrap();
        let green = self.hands.iter().map(|x| x.green).max().unwrap();
        let blue = self.hands.iter().map(|x| x.blue).max().unwrap();

        red * green * blue
    }
}

fn hand(s: &str) -> IResult<&str, Hand> {
    map(
        separated_list1(
            tag(", "),
            alt((
                map(terminated(u32, tag(" red")), Color::Red),
                map(terminated(u32, tag(" green")), Color::Green),
                map(terminated(u32, tag(" blue")), Color::Blue),
            )),
        ),
        Hand::new,
    ).parse(s)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                preceded(tag("Game "), u32),
                tag(": "),
                separated_list1(tag("; "), hand),
            ),
            |(id, hands)| Game { id, hands },
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    input.iter().filter(|x| x.is_possible()).map(|x| x.id).sum()
}

fn problem2(input: &Input) -> u32 {
    input.iter().map(|x| x.power()).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 8)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2286)
    }
}
