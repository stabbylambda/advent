use common::{
    answer, extensions::PointExt, grid::{Coord, Grid}, nom::parse_grid, read_input,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};
use std::collections::{BTreeMap, VecDeque};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('.'), |_| Tile::Track),
        map(char('S'), |_| Tile::Start),
        map(char('E'), |_| Tile::End),
        map(char('#'), |_| Tile::Wall),
    ))).parse(input);

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

fn get_all_saved(input: &Input, mut f: impl FnMut(u64) -> bool) -> BTreeMap<u64, u64> {
    let base_costs = get_base_costs(input);
    let mut all_saved = BTreeMap::new();

    for (start, start_cost) in &base_costs {
        for (end, end_cost) in &base_costs {
            let dist = start.manhattan(end) as u64;
            if f(dist) {
                if let Some(saved) = start_cost.checked_sub(*end_cost) {
                    let key = saved - dist;
                    if key > 0 {
                        all_saved.entry(key).and_modify(|x| *x += 1).or_insert(1);
                    }
                }
            }
        }
    }

    all_saved
}

fn problem1(input: &Input) -> u64 {
    let all_saved = get_all_saved(input, |x| x == 2);
    all_saved.iter().filter(|x| x.0 >= &100).map(|x| x.1).sum()
}

fn problem2(input: &Input) -> u64 {
    let all_saved = get_all_saved(input, |x| x <= 20);
    all_saved.iter().filter(|x| x.0 >= &100).map(|x| x.1).sum()
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::{get_all_saved, parse};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let all_saved = get_all_saved(&input, |x| x == 2);
        let expected = BTreeMap::from_iter([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]);
        assert_eq!(all_saved, expected)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = get_all_saved(&input, |x| x <= 20);
        let result: BTreeMap<_, _> = result.into_iter().filter(|x| x.0 >= 50).collect();
        assert_eq!(
            result,
            BTreeMap::from_iter([
                (50, 32),
                (52, 31),
                (54, 29),
                (56, 39),
                (58, 25),
                (60, 23),
                (62, 20),
                (64, 19),
                (66, 12),
                (68, 14),
                (70, 12),
                (72, 22),
                (74, 4),
                (76, 3),
            ])
        );
    }
}
