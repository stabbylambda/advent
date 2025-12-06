use common::{
    answer,
    grid::{CardinalDirection, Grid, Position},
    nom::parse_grid,
    read_input,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};
use std::collections::BTreeSet;

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<Tile>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Guard,
    Obstruction,
    Space,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('.'), |_| Tile::Space),
        map(char('^'), |_| Tile::Guard),
        map(char('#'), |_| Tile::Obstruction),
    ))).parse(input);

    result.unwrap().1
}

enum WalkResult {
    Leave(Vec<Position>),
    Loop,
}

fn walk(grid: &Grid<Tile>) -> WalkResult {
    let mut position = get_start(grid);
    let mut visited: Vec<_> = vec![];
    let mut turns: BTreeSet<_> = BTreeSet::new();
    visited.push(position);

    while let Some(next) = grid.get_neighbor(position.0, position.1) {
        if next.data == &Tile::Obstruction {
            position = position.turn_right();
            if !turns.insert(position) {
                // we've turned here before, so we're in a loop
                return WalkResult::Loop;
            }
        } else {
            position = position.step();
            visited.push(position);
        }
    }

    WalkResult::Leave(visited)
}

fn get_start(input: &Input) -> Position {
    input
        .iter()
        .find_map(|x| {
            (x.data == &Tile::Guard).then_some(Position::new(x.coords, CardinalDirection::North))
        })
        .unwrap()
}

fn problem1(input: &Input) -> usize {
    let WalkResult::Leave(visited) = walk(input) else {
        panic!("Something's wrong with the grid")
    };

    let unique: BTreeSet<_> = visited.iter().map(|x| x.0).collect();
    unique.len()
}

fn problem2(input: &Input) -> usize {
    let WalkResult::Leave(visited) = walk(input) else {
        panic!("Something's wrong with the grid")
    };

    let mut obstacles: BTreeSet<_> = BTreeSet::new();
    for pos in &visited {
        let mut grid = input.clone();
        if grid.get(pos.0).data == &Tile::Space {
            grid.set(pos.0, Tile::Obstruction);
        }

        if let WalkResult::Loop = walk(&grid) {
            obstacles.insert(pos.0);
        }
    }

    obstacles.len()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 41)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 6)
    }
}
