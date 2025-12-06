use std::collections::{HashMap, HashSet};

use common::{
    grid::{Coord, Grid, GridSquare},
    nom::{parse_grid, single_digit},
};
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::map,
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

#[derive(Clone, Copy, Debug)]
enum SchematicPart {
    Symbol(char),
    Number(u32),
    Blank,
}

type PartNumber = u32;
type Input = HashMap<(char, Coord), Vec<PartNumber>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        parse_grid(alt((
            map(char('.'), |_| SchematicPart::Blank),
            map(single_digit, SchematicPart::Number),
            map(one_of("!@#$%^&*+-=/\\"), SchematicPart::Symbol),
        ))),
        parse_schematic_parts,
    ).parse(input);

    result.unwrap().1
}

/** Get all the neighbors (including diagonal) that are symbols */
fn get_adjacent_symbols(x: &GridSquare<SchematicPart>) -> HashSet<(char, Coord)> {
    x.all_neighbors()
        .iter()
        .filter_map(|x| match x.data {
            SchematicPart::Symbol(c) => Some((*c, x.coords)),
            _ => None,
        })
        .collect()
}

fn parse_schematic_parts(input: Grid<SchematicPart>) -> Input {
    // maintain a cache for all the coordinates we've examined
    let mut examined: Vec<Coord> = vec![];

    // We ultimately want a list of adjacent numbers for each individual symbol
    let mut adjacencies: Input = HashMap::new();

    for x in input.iter() {
        // have we already been here?
        if examined.contains(&x.coords) {
            continue;
        }

        /*
        We'll go through the grid number-wise, not symbol-wise. This makes it easier to find the whole number
        (because we only have to move right once we find the first digit) and to check the adjacent symbols
        for each digit in the number. We'll know the unique symbols because the hashset will be populated with
        the symbol itself *and* the coordinates so that we won't double count.
        */
        if let SchematicPart::Number(n) = x.data {
            let mut current = x;
            let mut num = *n;
            let mut adjacent_symbols = get_adjacent_symbols(&x);

            // go right until we don't have a number anymore
            while let Some(SchematicPart::Number(next)) = current.neighbors().east.map(|x| x.data) {
                // multiply by 10 then add the next number and move the cursor over
                num = (num * 10) + *next;
                current = current.neighbors().east.unwrap();

                // are we adjacent to a symbol here?
                adjacent_symbols.extend(get_adjacent_symbols(&current));

                // mark that we've seen the successor square and don't need to visit it again
                examined.push(current.coords);
            }

            // if we don't have any adjacent symbols, this number doesn't matter
            if adjacent_symbols.is_empty() {
                continue;
            }

            // add this number to the adjacency list for each symbol
            for adj in adjacent_symbols {
                adjacencies
                    .entry(adj)
                    .and_modify(|v| v.push(num))
                    .or_insert(vec![num]);
            }
        }
    }
    adjacencies
}

fn problem1(input: &Input) -> u32 {
    input.values().flatten().sum()
}

fn problem2(input: &Input) -> u32 {
    input
        .iter()
        // get all the gears (marked as an asterisk) with two adjacent part numbers
        .filter(|((c, _), parts)| *c == '*' && parts.len() == 2)
        // get the gear ratios
        .map(|(_, v)| v.iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 4361)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 467835)
    }
}
