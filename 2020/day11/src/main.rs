use std::fmt::Display;

use common::{
    grid::{Grid, GridSquare},
    nom::parse_grid,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Grid<Tile>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => ".",
                Tile::Empty => "L",
                Tile::Occupied => "#",
            }
        )
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('#'), |_| Tile::Occupied),
        map(char('L'), |_| Tile::Empty),
        map(char('.'), |_| Tile::Floor),
    )))(input);

    result.unwrap().1
}

fn tick1(map: &Grid<Tile>) -> Grid<Tile> {
    let mut new_map = map.clone();

    for x in map.into_iter() {
        // count all the neighbors
        let occupied_neighbors = x
            .all_neighbors()
            .into_iter()
            .filter(|x| *x.data == Tile::Occupied)
            .count();

        let new_seat = match x.data {
            Tile::Empty if occupied_neighbors == 0 => Tile::Occupied,
            Tile::Occupied if occupied_neighbors >= 4 => Tile::Empty,
            _ => *x.data,
        };
        new_map.set(x.coords, new_seat);
    }

    new_map
}

/*
This is, perhaps, the most ridiculous way to do this.
  - I'm starting at the coordinates of a square
  - Find the maximum absolute distance to the edge of the area
  - For every delta from 1 to that maximum absolute distance, go out along each path and then find the first tile that is a seat
 */
fn visible_neighbor_count(map: &Grid<Tile>, (x, y): (usize, usize)) -> u32 {
    let is_occupied = |s: &GridSquare<Tile>| s.data == &Tile::Occupied;
    let is_seat = |s: &GridSquare<Tile>| matches!(s.data, Tile::Empty | Tile::Occupied);

    let max_dx = x.max(x.abs_diff(map.width));
    let max_dy = y.max(y.abs_diff(map.height));
    let max_d = max_dx.max(max_dy);

    let mut north = None;
    let mut south = None;
    let mut east = None;
    let mut west = None;
    let mut north_west = None;
    let mut north_east = None;
    let mut south_west = None;
    let mut south_east = None;

    for d in 1..max_d {
        // check the bounds to make sure we don't grab tiles that don't exist
        let x_minus_d = x.checked_sub(d);
        let x_plus_d = (x + d < map.width).then_some(x + d);
        let y_minus_d = y.checked_sub(d);
        let y_plus_d = (y + d < map.height).then_some(y + d);

        // in each direction, only check more seats if we haven't already found one
        north = north.or(y_minus_d.map(|y| map.get((x, y))).filter(is_seat));
        south = south.or(y_plus_d.map(|y| map.get((x, y))).filter(is_seat));
        west = west.or(x_minus_d.map(|x| map.get((x, y))).filter(is_seat));
        east = east.or(x_plus_d.map(|x| map.get((x, y))).filter(is_seat));
        north_west = north_west.or(x_minus_d.zip(y_minus_d).map(|c| map.get(c)).filter(is_seat));
        north_east = north_east.or(x_plus_d.zip(y_minus_d).map(|c| map.get(c)).filter(is_seat));
        south_west = south_west.or(x_minus_d.zip(y_plus_d).map(|c| map.get(c)).filter(is_seat));
        south_east = south_east.or(x_plus_d.zip(y_plus_d).map(|c| map.get(c)).filter(is_seat));
    }

    // count how many seats are actually occupied
    north.filter(is_occupied).is_some() as u32
        + south.filter(is_occupied).is_some() as u32
        + east.filter(is_occupied).is_some() as u32
        + west.filter(is_occupied).is_some() as u32
        + north_east.filter(is_occupied).is_some() as u32
        + north_west.filter(is_occupied).is_some() as u32
        + south_east.filter(is_occupied).is_some() as u32
        + south_west.filter(is_occupied).is_some() as u32
}

fn tick2(map: &Grid<Tile>) -> Grid<Tile> {
    let mut new_map = map.clone();

    for s in map.into_iter() {
        let occupied_neighbors = visible_neighbor_count(map, s.coords);
        let new_seat = match s.data {
            Tile::Empty if occupied_neighbors == 0 => Tile::Occupied,
            Tile::Occupied if occupied_neighbors >= 5 => Tile::Empty,
            _ => *s.data,
        };
        new_map.set(s.coords, new_seat);
    }

    new_map
}

fn simulate(input: &Input, f: impl Fn(&Grid<Tile>) -> Grid<Tile>) -> usize {
    let mut map = input.clone();

    loop {
        let new_map = f(&map);
        if new_map.points == map.points {
            return new_map
                .into_iter()
                .filter(|x| *x.data == Tile::Occupied)
                .count();
        } else {
            map = new_map;
        }
    }
}

fn problem1(input: &Input) -> usize {
    simulate(input, tick1)
}

fn problem2(input: &Input) -> usize {
    simulate(input, tick2)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 37)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 26)
    }
}
