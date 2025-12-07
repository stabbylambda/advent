use std::{collections::BTreeMap, time::Instant};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    answer!(answer1);
    answer!(answer2);
}

type Input = (Vec<bool>, Vec<Vec<bool>>);

fn parse(input: &str) -> Input {
    let pixels = |s| many1(alt((map(char('#'), |_| true), map(char('.'), |_| false)))).parse(s);
    let result: IResult<&str, Input> =
        separated_pair(pixels, tag("\n\n"), separated_list1(newline, pixels)).parse(input);

    result.unwrap().1
}

// these are in "reverse" order so we can bit shift correctly
const NEIGHBORS: [(i32, i32); 9] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, 0),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn get_pixels((y, x): &(i32, i32)) -> Vec<(i32, i32)> {
    NEIGHBORS.iter().map(|(dy, dx)| (y + dy, x + dx)).collect()
}

fn enhance(p: &(i32, i32), values: &BTreeMap<(i32, i32), bool>, default: bool) -> usize {
    get_pixels(p)
        .iter()
        .map(|k| values.get(k).copied().unwrap_or(default) as usize)
        .enumerate()
        .fold(0, |acc, (idx, x)| acc | (x << idx))
}

fn problem(input: &Input) -> (usize, usize) {
    let (algorithm, image) = input;
    let mut image: BTreeMap<(i32, i32), bool> = image
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .enumerate()
                .map(move |(x, c)| ((y as i32, x as i32), *c))
        })
        .collect();

    let mut answers = vec![];

    for round in 0..50 {
        let t = Instant::now();
        let default = if round % 2 == 1 { algorithm[0] } else { false };

        // consider all the pixels in the image and their neighbors
        let new_image: BTreeMap<(i32, i32), bool> = image
            .keys()
            .flat_map(get_pixels)
            .map(|p| {
                let index = enhance(&p, &image, default);
                (p, algorithm[index])
            })
            .collect();

        // count the number of pixels that are on
        let on = new_image.values().filter(|x| **x).count();
        println!("{round} {on} ({:?})", t.elapsed());
        answers.push(on);

        image = new_image;
    }

    (answers[1], answers[49])
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (result1, result2) = problem(&input);
        assert_eq!(result1, 35);
        assert_eq!(result2, 3351);
    }
}
