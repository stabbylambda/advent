use nom::{IResult, Parser};

use common::grid::{Grid, GridSquare};
use common::nom::parse_grid;
use common::{answer, read_input, GridTile};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

#[derive(Copy, Clone, PartialEq, Eq, GridTile)]
enum Tile {
    #[tile('@')]
    Roll,
    #[tile('.')]
    Empty,
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(Tile::parser()).parse(input);
    result.unwrap().1
}

fn removable<'a>(grid: &'a Grid<Tile>) -> Vec<GridSquare<'a, Tile>> {
    let v = grid
        .iter()
        .filter(|x| x.data == &Tile::Roll)
        .filter(|x| {
            let rolls = x
                .all_neighbors()
                .iter()
                .filter(|n| n.data == &Tile::Roll)
                .count();
            rolls < 4
        })
        .collect::<Vec<_>>();

    v
}

fn problem1(x: &Input) -> usize {
    removable(x).len()
}

fn remove_rolls(count: usize, grid: &Grid<Tile>) -> usize {
    let r = removable(grid);
    let new_count = r.len();
    if new_count == 0 {
        return count;
    }

    let mut new_grid = grid.clone();
    for x in r {
        new_grid.set(x.coords, Tile::Empty);
    }

    remove_rolls(count + new_count, &new_grid)
}

fn problem2(x: &Input) -> usize {
    remove_rolls(0, x)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 43)
    }
}
