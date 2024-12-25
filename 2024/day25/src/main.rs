use common::{grid::Grid, nom::parse_grid};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, multi::separated_list1, IResult,
};
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");
}

type Input = Vec<Grid<char>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(tag("\n\n"), parse_grid(alt((char('#'), char('.')))))(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let mut keys = vec![];
    let mut locks = vec![];
    for grid in input {
        let is_lock = grid.points.first().unwrap().iter().all(|x| *x == '#');
        let grid = grid.transpose();
        let c: Vec<usize> = grid
            .points
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|x| if is_lock { **x == '.' } else { **x == '#' })
                    .count()
                    - 1
            })
            .collect();

        if is_lock {
            locks.push(c);
        } else {
            keys.push(c);
        }
    }

    locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|(lock, key)| fits(&key[..], &lock[..]))
        .count()
}

fn fits(key: &[usize], lock: &[usize]) -> bool {
    lock.iter().zip(key).all(|(l, k)| l >= k)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 3)
    }
}
