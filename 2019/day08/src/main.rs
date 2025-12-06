use std::fmt::Display;

use common::nom::single_digit;
use nom::{multi::many1, IResult, Parser};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer:\n\n{answer}");
}

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(single_digit).parse(input);

    result.unwrap().1
}

#[derive(Debug, PartialEq, Eq)]
struct Layer(Vec<Vec<Cell>>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Black,
    White,
    Transparent,
}

impl From<u32> for Cell {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            2 => Self::Transparent,
            _ => unreachable!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell = match self {
            Self::White => "#",
            _ => " ",
        };
        write!(f, "{cell}")
    }
}

impl Layer {
    fn count(&self, cell: Cell) -> usize {
        self.0.iter().flatten().filter(|x| **x == cell).count()
    }

    fn zeros(&self) -> usize {
        self.count(Cell::Black)
    }

    fn ones(&self) -> usize {
        self.count(Cell::White)
    }

    fn twos(&self) -> usize {
        self.count(Cell::Transparent)
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn get_final_image(input: &[Layer], width: usize, height: usize) -> Layer {
    let result = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    input
                        .iter()
                        .map(|l| l.0[y][x])
                        .find(|x| *x != Cell::Transparent)
                        .unwrap_or(Cell::Black)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Layer(result)
}

fn get_layers(input: &[u32], width: usize, height: usize) -> Vec<Layer> {
    let limit = width * height;
    input
        .chunks(limit)
        .map(|x| {
            Layer(
                x.chunks(width)
                    .map(|x| x.iter().map(|x| Cell::from(*x)).collect())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn problem1(input: &Input) -> usize {
    let layers = get_layers(input, 25, 6);
    let layer = layers.iter().min_by_key(|x| x.zeros()).unwrap();

    layer.ones() * layer.twos()
}

fn problem2(input: &Input) -> Layer {
    let layers = get_layers(input, 25, 6);
    get_final_image(&layers, 25, 6)
}

#[cfg(test)]
mod test {
    use crate::{get_final_image, get_layers, parse, Cell, Layer};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = get_layers(&input, 3, 2);
        assert_eq!(
            result,
            vec![
                Layer(vec![
                    vec![Cell::Black, Cell::Black, Cell::Black],
                    vec![Cell::Black, Cell::Black, Cell::Black],
                ]),
                Layer(vec![
                    vec![Cell::Black, Cell::Black, Cell::Black],
                    vec![Cell::Black, Cell::White, Cell::Transparent],
                ]),
            ]
        )
    }

    #[test]
    fn second() {
        let input = "0222112222120000";
        let input = parse(input);
        let input = get_layers(&input, 2, 2);
        let result = get_final_image(&input, 2, 2);
        assert_eq!(
            result,
            Layer(vec![
                vec![Cell::Black, Cell::White],
                vec![Cell::White, Cell::Black]
            ])
        )
    }
}
