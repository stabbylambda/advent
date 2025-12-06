use common::{
    answer,
    grid::{Grid, GridSquare},
    nom::parse_grid,
    read_input,
};
use nom::{branch::alt, character::complete::char, IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<char>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        parse_grid(alt((char('X'), char('M'), char('A'), char('S')))).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    fn spells(x: &GridSquare<'_, char>, s: &str, dir: Option<usize>) -> usize {
        match s.chars().next() {
            Some(c) if &c != x.data => 0,
            Some('S') if x.data == &'S' => 1,
            _ => {
                let n = x.all_neighbors();
                n.iter_all()
                    .enumerate()
                    .filter(|(d, _)| match dir {
                        // if we're going in a direction, we have to keep going
                        Some(x) => x == *d,
                        // otherwise all candidates are fine
                        None => true,
                    })
                    .filter_map(|(d, x)| x.map(|x| spells(&x, &s[1..], Some(d))))
                    .sum()
            }
        }
    }
    input.iter().map(|x| spells(&x, "XMAS", None)).sum()
}

fn problem2(input: &Input) -> usize {
    fn get_corners(a: &GridSquare<char>) -> Option<String> {
        let n = a.all_neighbors();
        let a = a.data;
        let nw = n.north_west.map(|x| x.data);
        let ne = n.north_east.map(|x| x.data);
        let sw = n.south_west.map(|x| x.data);
        let se = n.south_east.map(|x| x.data);

        // create the string of all the corners around the center
        nw.and_then(|nw| {
            ne.and_then(|ne| se.and_then(|se| sw.map(|sw| format!("{nw}{ne}{a}{se}{sw}"))))
        })
    }
    input
        .iter()
        .flat_map(|a| get_corners(&a))
        .filter(|x| matches!(x.as_str(), "MSASM" | "MMASS" | "SMAMS" | "SSAMM"))
        .count()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 18)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 9)
    }
}
