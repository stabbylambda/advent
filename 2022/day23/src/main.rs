use std::collections::{BTreeSet, HashMap};

use nom::{
    branch::alt,
    character::complete::{char, newline},
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let mut input = parse(input);

    let answer = problem1(&mut input.clone());
    println!("problem 1 answer: {answer}");

    let answer = problem2(&mut input);
    println!("problem 2 answer: {answer}");
}

type Input = BTreeSet<(i64, i64)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Vec<Vec<char>>> =
        separated_list1(newline, many1(alt((char('#'), char('.'))))).parse(input);

    result
        .unwrap()
        .1
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, c)| (*c == '#').then_some((x as i64, y as i64)))
        })
        .collect()
}

#[derive(Debug)]
struct Proposal {
    current: (i64, i64),
    proposed: (i64, i64),
}

fn get_proposal(round: usize, input: &Input, elf @ (x, y): (i64, i64)) -> Option<Proposal> {
    // first half, let's check all our neighbors
    let north = input.contains(&(x, y - 1));
    let south = input.contains(&(x, y + 1));
    let west = input.contains(&(x - 1, y));
    let east = input.contains(&(x + 1, y));

    let north_west = input.contains(&(x - 1, y - 1));
    let north_east = input.contains(&(x + 1, y - 1));
    let south_west = input.contains(&(x - 1, y + 1));
    let south_east = input.contains(&(x + 1, y + 1));

    let neighbors = [
        north, south, east, west, north_west, north_east, south_west, south_east,
    ];
    if neighbors.iter().all(|x| !x) {
        // println!("{elf:?} has no neighbors");
        return None;
    }

    // if we do have neighbors, we need to propose a direction
    let proposals = [
        (!north && !north_east && !north_west, (x, y - 1)),
        (!south && !south_east && !south_west, (x, y + 1)),
        (!west && !north_west && !south_west, (x - 1, y)),
        (!east && !north_east && !south_east, (x + 1, y)),
    ];

    for i in 0..4 {
        let (free, pos) = proposals[(round + i) % 4];
        // println!("{free} {elf:?} -> {pos:?}");
        if free {
            return Some(Proposal {
                current: elf,
                proposed: pos,
            });
        }
    }

    None
}

fn get_proposals(input: &Input, round: usize) -> HashMap<(i64, i64), Vec<(i64, i64)>> {
    // get the proposals
    // println!("Proposals for round {round}");
    let proposals: Vec<Proposal> = input
        .iter()
        .filter_map(|e| get_proposal(round, input, *e))
        .collect();

    // put them into a map so we can group them
    let mut map: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for p in &proposals {
        map.entry(p.proposed)
            .and_modify(|v| v.push(p.current))
            .or_insert_with(|| vec![p.current]);
    }

    map
}

fn problem1(input: &mut Input) -> u64 {
    for round in 0..=9 {
        let proposals = get_proposals(input, round);

        // no point in going further, nobody is moving
        if proposals.is_empty() {
            break;
        }

        // now move them around
        proposals
            .iter()
            .filter(|(_, current)| current.len() == 1)
            .for_each(|(target, current)| {
                input.remove(&current[0]);
                input.insert(*target);
            });

        // print_map(input);
    }

    let min_y = input.iter().map(|(_, y)| y).min().unwrap();
    let max_y = input.iter().map(|(_, y)| y).max().unwrap();
    let min_x = input.iter().map(|(x, _)| x).min().unwrap();
    let max_x = input.iter().map(|(x, _)| x).max().unwrap();

    let x_range = 1 + min_x.abs_diff(*max_x);
    let y_range = 1 + min_y.abs_diff(*max_y);

    x_range * y_range - (input.len() as u64)
}

#[allow(dead_code)]
fn print_map(input: &Input) {
    let &min_y = input.iter().map(|(_, y)| y).min().unwrap().min(&0);
    let &max_y = input.iter().map(|(_, y)| y).max().unwrap().max(&4);
    let &min_x = input.iter().map(|(x, _)| x).min().unwrap().min(&0);
    let &max_x = input.iter().map(|(x, _)| x).max().unwrap().max(&4);

    println!("=====================");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if input.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }

    println!("=====================");
}

fn problem2(input: &mut Input) -> usize {
    for round in 0.. {
        let proposals = get_proposals(input, round);

        // no point in going further, nobody is moving
        if proposals.is_empty() {
            return round + 1;
        } else {
            println!("There were {} proposals", proposals.len());
        }

        // now move them around
        proposals
            .iter()
            .filter(|(_, current)| current.len() == 1)
            .for_each(|(target, current)| {
                input.remove(&current[0]);
                input.insert(*target);
            });
    }
    panic!()
}

#[cfg(test)]
mod test {

    use crate::{parse, print_map, problem1, problem2};

    #[test]
    fn mini() {
        let input = ".....
..##.
..#..
.....
..##.
.....";
        let mut input = parse(input);
        print_map(&input);
        let result = problem1(&mut input);
        assert_eq!(result, 25)
    }

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let mut input = parse(input);
        let result = problem1(&mut input);
        assert_eq!(result, 110)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let mut input = parse(input);
        let result = problem2(&mut input);
        assert_eq!(result, 20)
    }
}
