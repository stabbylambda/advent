use common::{
    answer,
    extensions::PointExt,
    grid::{CardinalDirection, Coord, Grid},
    nom::{parse_grid, single_digit},
    read_input,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use std::{collections::{BinaryHeap, HashMap}, hash::Hash};

fn main() {
    let keypads = include_str!("../keypads.txt");
    let keypads = parse_keypads(keypads);

    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&keypads, &input));
    answer!(problem2(&keypads, &input));
}

type Input = Vec<Vec<NumericKeypad>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(single_digit, |x| NumericKeypad::Number(x as u64)),
            map(char('A'), |_| NumericKeypad::Accept),
        ))),
    ).parse(input);

    result.unwrap().1
}

type Keypads = (Keypad<NumericKeypad>, Keypad<DirectionalKeypad>);

fn parse_keypads(input: &str) -> Keypads {
    let result: IResult<&str, Keypads> = separated_pair(
        map(
            parse_grid(alt((
                map(single_digit, |x| Some(NumericKeypad::Number(x as u64))),
                map(char('A'), |_| Some(NumericKeypad::Accept)),
                map(char(' '), |_| None),
            ))),
            Keypad::new,
        ),
        tag("\n\n"),
        map(
            parse_grid(alt((
                map(char(' '), |_| None),
                map(char('A'), |_| Some(DirectionalKeypad::Accept)),
                map(char('^'), |_| Some(DirectionalKeypad::Up)),
                map(char('v'), |_| Some(DirectionalKeypad::Down)),
                map(char('<'), |_| Some(DirectionalKeypad::Left)),
                map(char('>'), |_| Some(DirectionalKeypad::Right)),
            ))),
            Keypad::new,
        ),
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone)]
struct Keypad<T: Hash> {
    grid: Grid<Option<T>>,
    cache: HashMap<(T, T, u64), usize>,
}

impl<T: Hash> Keypad<T> {
    fn new(grid: Grid<Option<T>>) -> Self {
        Self {
            grid,
            cache: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum NumericKeypad {
    Number(u64),
    Accept,
}

impl NumericKeypad {
    fn to_number(seq: &[NumericKeypad]) -> usize {
        let mut numeric_part = 0_usize;
        for x in seq {
            match x {
                NumericKeypad::Accept => {}
                // skip leading zeros
                NumericKeypad::Number(0) if numeric_part == 0 => {}
                NumericKeypad::Number(x) => {
                    numeric_part = (numeric_part * 10) + *x as usize;
                }
            }
        }
        numeric_part
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum DirectionalKeypad {
    Up,
    Down,
    Left,
    Right,
    Accept,
}
impl From<CardinalDirection> for DirectionalKeypad {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => DirectionalKeypad::Up,
            CardinalDirection::South => DirectionalKeypad::Down,
            CardinalDirection::East => DirectionalKeypad::Right,
            CardinalDirection::West => DirectionalKeypad::Left,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct KeypadSearchState {
    current: Coord,
    dist: usize,
    sequence: Vec<DirectionalKeypad>,
    best: usize,
}

impl PartialOrd for KeypadSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for KeypadSearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.best.cmp(&self.best)
    }
}

const DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::North,
    CardinalDirection::South,
    CardinalDirection::East,
    CardinalDirection::West,
];

fn best_directional_sequence(
    keypad: &mut Keypad<DirectionalKeypad>,
    from: DirectionalKeypad,
    to: DirectionalKeypad,
    robots: u64,
) -> usize {
    if let Some(cached) = keypad.cache.get(&(from, to, robots)) {
        return *cached;
    }

    let grid = keypad.grid.clone();

    let from_square = grid.find(Some(from)).unwrap();
    let to_square = grid.find(Some(to)).unwrap();

    let state = KeypadSearchState {
        current: from_square.coords,
        dist: from_square.coords.manhattan(&to_square.coords),
        sequence: vec![],
        best: 0,
    };

    let mut queue = BinaryHeap::from_iter([state]);
    let mut best = usize::MAX;

    while let Some(current) = queue.pop() {
        let key = grid.get(current.current);

        // did we find the key we want?
        if key.data.unwrap() == to {
            let robot_sequence = best_robot_sequence(keypad, &current.sequence, robots - 1);
            best = best.min(robot_sequence);
            continue;
        }

        for dir in DIRECTIONS {
            if let Some(next) = key.get_neighbor(dir) {
                // we can't step into the gap
                if next.data.is_none() {
                    continue;
                }

                // only move closer to the target, never away
                let new_dist = next.coords.manhattan(&to_square.coords);
                if new_dist >= current.dist {
                    continue;
                }

                let mut new_state = current.clone();
                new_state.sequence.push(dir.into());
                new_state.current = next.coords;
                new_state.best = best_robot_sequence(keypad, &new_state.sequence, robots - 1);
                new_state.dist = new_dist;
                queue.push(new_state);
            }
        }
    }

    keypad.cache.insert((from, to, robots), best);
    best
}

fn best_robot_sequence(
    keypad: &mut Keypad<DirectionalKeypad>,
    sequence: &[DirectionalKeypad],
    robots: u64,
) -> usize {
    let mut sequence = sequence.to_vec();
    sequence.push(DirectionalKeypad::Accept);
    // if we got to our own input, this is it
    if robots == 0 {
        return sequence.len();
    }

    let (_, len) =
        sequence
            .iter()
            .fold((DirectionalKeypad::Accept, 0_usize), |(from, len), &to| {
                let len = len + best_directional_sequence(keypad, from, to, robots);
                (to, len)
            });

    len
}

fn best_numeric_sequence(
    keypad: &mut Keypad<NumericKeypad>,
    robot_keypad: &mut Keypad<DirectionalKeypad>,
    from: NumericKeypad,
    to: NumericKeypad,
    robots: u64,
) -> usize {
    if let Some(cached) = keypad.cache.get(&(from, to, robots)) {
        return *cached;
    }

    let from_square = keypad.grid.find(Some(from)).unwrap();

    let state = KeypadSearchState {
        current: from_square.coords,
        dist: 0,
        sequence: vec![],
        best: 0,
    };

    let mut queue = BinaryHeap::from_iter([state]);

    while let Some(current) = queue.pop() {
        let key = keypad.grid.get(current.current);

        // did we find the key we want?
        if key.data.unwrap() == to {
            return current.best;
        }

        for dir in DIRECTIONS {
            if let Some(next) = key.get_neighbor(dir) {
                // we can't step into the gap
                if next.data.is_none() {
                    continue;
                }

                // only move closer to the target, never away

                let mut new_state = current.clone();
                new_state.sequence.push(dir.into());
                new_state.current = next.coords;
                new_state.dist = 0;
                new_state.best = best_robot_sequence(robot_keypad, &new_state.sequence, robots);
                queue.push(new_state);
            }
        }
    }

    unreachable!()
}

fn solve(keypads: &Keypads, input: &Input, robots: u64) -> usize {
    let (mut numeric, mut robot) = keypads.clone();
    let mut result = 0;

    for code in input {
        let (_, shortest) =
            code.iter()
                .fold((NumericKeypad::Accept, 0_usize), |(from, len), &to| {
                    let len =
                        len + best_numeric_sequence(&mut numeric, &mut robot, from, to, robots);
                    (to, len)
                });

        result += NumericKeypad::to_number(code) * shortest;
    }

    result
}

fn problem1(keypads: &Keypads, input: &Input) -> usize {
    solve(keypads, input, 2)
}

fn problem2(keypads: &Keypads, input: &Input) -> usize {
    solve(keypads, input, 25)
}

#[cfg(test)]
mod test {
    use crate::{parse, parse_keypads, problem1};
    #[test]
    fn first() {
        let keypads = include_str!("../keypads.txt");
        let keypads = parse_keypads(keypads);
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&keypads, &input);
        assert_eq!(result, 126384)
    }
}
