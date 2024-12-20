use common::{
    grid::{CardinalDirection, Coord, Grid},
    nom::parse_grid,
};
use itertools::all;
use nom::{
    branch::alt,
    character::complete::{char, newline, u64},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    time::Instant,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('.'), |_| Tile::Track),
        map(char('S'), |_| Tile::Start),
        map(char('E'), |_| Tile::End),
        map(char('#'), |_| Tile::Wall),
    )))(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Track,
    Start,
    End,
    Wall,
}

fn get_base_costs(input: &Input) -> BTreeMap<Coord, u64> {
    let end = input.find(Tile::End).map(|x| x.coords).unwrap();

    let mut base_cost: BTreeMap<Coord, u64> = BTreeMap::new();
    let mut queue: VecDeque<(Coord, u64)> = VecDeque::from_iter([(end, 0)]);

    while let Some((current, picoseconds)) = queue.pop_front() {
        base_cost.insert(current, picoseconds);

        for next in input.get(current).neighbors().iter() {
            if (next.data == &Tile::Track || next.data == &Tile::Start)
                && !base_cost.contains_key(&next.coords)
            {
                queue.push_back((next.coords, picoseconds + 1));
            }
        }
    }

    base_cost
}

fn problem1(input: &Input) -> u64 {
    let base_costs = get_base_costs(input);

    let mut all_saved = BTreeMap::new();

    for current in input
        .iter()
        .filter(|x| x.data == &Tile::Track || x.data == &Tile::Start)
    {
        for dir in [
            CardinalDirection::North,
            CardinalDirection::South,
            CardinalDirection::East,
            CardinalDirection::West,
        ] {
            // get neighbors that are walls
            let Some(wall_neighbor) = input
                .get_neighbor(current.coords, dir)
                .filter(|x| x.data == &Tile::Wall)
            else {
                continue;
            };

            let Some(over_the_wall) = input
                .get_neighbor(wall_neighbor.coords, dir)
                .filter(|x| x.data == &Tile::Track || x.data == &Tile::End)
            else {
                continue;
            };

            let saved = (base_costs[&current.coords] as isize)
                - (base_costs[&over_the_wall.coords] as isize)
                - 2;

            if saved > 0 {
                all_saved
                    .entry(saved as usize)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }
    }

    all_saved.iter().filter(|x| x.0 >= &100).map(|x| x.1).sum()
}

fn problem2(input: &Input) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
