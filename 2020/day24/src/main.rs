use std::collections::BTreeSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Vec<(i32, i32)>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(tag("se"), |_| (1, -1)),
            map(tag("sw"), |_| (0, -1)),
            map(tag("ne"), |_| (0, 1)),
            map(tag("nw"), |_| (-1, 1)),
            map(tag("e"), |_| (1, 0)),
            map(tag("w"), |_| (-1, 0)),
        ))),
    ).parse(input);

    result.unwrap().1
}

fn get_neighbors(x: i32, y: i32) -> BTreeSet<(i32, i32)> {
    let neighbors = [(1, -1), (0, -1), (0, 1), (-1, 1), (1, 0), (-1, 0)];
    neighbors.iter().map(|(dx, dy)| (x + dx, y + dy)).collect()
}

fn get_black_tiles(input: &Input) -> BTreeSet<(i32, i32)> {
    let results: Vec<(i32, i32)> = input
        .iter()
        .map(|p| p.iter().fold((0, 0), |(x, y), (dx, dy)| (x + *dx, y + *dy)))
        .collect();

    let mut set: BTreeSet<(i32, i32)> = BTreeSet::new();
    for r in &results {
        if set.contains(r) {
            set.remove(r);
        } else {
            set.insert(*r);
        }
    }

    set
}

fn problem1(input: &Input) -> usize {
    let tiles = get_black_tiles(input);
    tiles.len()
}

fn problem2(input: &Input) -> usize {
    let mut tiles = get_black_tiles(input);
    let mut min_x = tiles.iter().map(|x| x.0).min().unwrap();
    let mut max_x = tiles.iter().map(|x| x.0).max().unwrap();
    let mut min_y = tiles.iter().map(|x| x.1).min().unwrap();
    let mut max_y = tiles.iter().map(|x| x.1).max().unwrap();

    for _day in 0..100 {
        let mut new_tiles = BTreeSet::new();

        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let is_black = tiles.contains(&(x, y));
                let black_neighbors = get_neighbors(x, y).intersection(&tiles).count();

                let black_to_black = is_black && (black_neighbors == 1 || black_neighbors == 2);
                let white_to_black = !is_black && black_neighbors == 2;

                if black_to_black || white_to_black {
                    new_tiles.insert((x, y));

                    // update the bounds of the tile grid so we don't have to iterate again
                    min_x = x.min(min_x);
                    max_x = x.max(max_x);
                    min_y = y.min(min_y);
                    max_y = y.max(max_y);
                }
            }
        }

        tiles = new_tiles;
    }

    tiles.len()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 10)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2208)
    }
}
