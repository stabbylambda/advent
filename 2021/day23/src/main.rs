use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

use nom::{
    branch::alt,
    character::complete::{char, newline, one_of},
    combinator::map,
    multi::{many1, separated_list0},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem(&input);
    println!("problem 1 answer: {answer}");

    let input = include_str!("../input2.txt");
    let input = parse(input);

    let answer = problem(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Amphipods;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list0(
            newline,
            many1(alt((
                map(one_of(" .#"), |_| None),
                map(char('A'), |_| Some(Amphipod::Amber)),
                map(char('B'), |_| Some(Amphipod::Bronze)),
                map(char('C'), |_| Some(Amphipod::Copper)),
                map(char('D'), |_| Some(Amphipod::Desert)),
            ))),
        ),
        |input| {
            // all we really care about is the positions of the amphipods
            let amphipods: Vec<Amphipod> = input
                .iter()
                .skip(2)
                .flat_map(|row| row.iter().filter_map(|x| *x).collect_vec())
                .collect();

            Amphipods::new(&amphipods)
        },
    )(input);

    result.unwrap().1
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl Amphipod {
    fn energy(&self, steps: usize) -> usize {
        steps * 10_usize.pow(*self as u32)
    }

    fn from_idx(value: usize) -> Option<Amphipod> {
        match value {
            0 => Some(Amphipod::Amber),
            1 => Some(Amphipod::Bronze),
            2 => Some(Amphipod::Copper),
            3 => Some(Amphipod::Desert),

            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Room(usize, Vec<Option<Amphipod>>);

impl Room {
    fn is_done(&self) -> bool {
        self.1.iter().all(|x| *x == Amphipod::from_idx(self.0))
    }

    fn has_mismatched_amphipods(&self) -> bool {
        let expected = Amphipod::from_idx(self.0).unwrap();
        self.1.iter().any(|x| x.is_some_and(|a| a != expected))
    }

    fn topmost_amphipod(&self) -> (Amphipod, usize) {
        self.1
            .iter()
            .enumerate()
            .find_map(|(r, a)| a.map(|a| (a, r)))
            .unwrap()
    }

    fn matching_hallway_index(&self) -> usize {
        2 + self.0 * 2
    }

    fn is_open_to(&self, amphipod: Amphipod) -> bool {
        let expected = Amphipod::from_idx(self.0).unwrap();
        amphipod == expected
            && self.1.iter().all(|x| match x {
                Some(a) => *a == expected,
                None => true,
            })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Amphipods {
    hallway: [Option<Amphipod>; 11],
    rooms: [Room; 4],
}

impl Amphipods {
    fn new(amphipods: &[Amphipod]) -> Self {
        let mut s = Self::default();
        for x in 0..4 {
            s.rooms[x] = Room(x, vec![]);
        }

        for row in amphipods.chunks(4) {
            for (i, a) in row.iter().enumerate() {
                s.rooms[i].1.push(Some(*a));
            }
        }

        s
    }

    fn is_room_done(&self, idx: usize) -> bool {
        self.rooms[idx].is_done()
    }

    fn is_done(&self) -> bool {
        (0..4).all(|x| self.is_room_done(x))
    }

    fn is_hallway_clear(&self, from: usize, to: usize) -> bool {
        let (min_idx, max_idx) = if from < to {
            (from + 1, to)
        } else {
            (to, from - 1)
        };

        (min_idx..max_idx).all(|x| self.hallway[x].is_none())
    }

    fn open_hallway_locations(&self, room: &Room) -> Vec<usize> {
        let hallway_index = room.matching_hallway_index();

        // walk left and right until we get blocked
        let left = (0..hallway_index)
            .rev()
            .take_while(|x| self.hallway[*x].is_none());

        let right =
            (hallway_index + 1..self.hallway.len()).take_while(|x| self.hallway[*x].is_none());

        left.chain(right).collect_vec()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    amphipods: Amphipods,
    energy_spent: usize,
}

impl State {
    fn all_transitions(&self) -> Vec<State> {
        let mut transitions = vec![];
        transitions.extend(self.room_to_hallway());
        transitions.extend(self.hallway_to_room());

        transitions
    }

    fn hallway_to_room(&self) -> Vec<State> {
        self.amphipods
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(hallway_idx, a)| a.map(|amphipod| (hallway_idx, amphipod)))
            .filter_map(|(from, amphipod)| {
                let room_idx = amphipod as usize;
                let room = &self.amphipods.rooms[room_idx];
                let to = room.matching_hallway_index();

                if !room.is_open_to(amphipod) {
                    return None;
                }

                if !self.amphipods.is_hallway_clear(from, to) {
                    return None;
                }

                room.1
                    .iter()
                    .rposition(|x| x.is_none())
                    .and_then(|depth| self.swap_places(amphipod, from, room_idx, depth))
            })
            .collect()
    }

    fn room_to_hallway(&self) -> Vec<State> {
        self.amphipods
            .rooms
            .iter()
            // only consider rooms that aren't done yet
            .filter(|room| room.has_mismatched_amphipods())
            .flat_map(|room| {
                let room_idx = room.0;
                let (amphipod, depth) = room.topmost_amphipod();

                self.amphipods
                    .open_hallway_locations(room)
                    .into_iter()
                    .filter_map(move |hallway_idx| {
                        self.swap_places(amphipod, hallway_idx, room_idx, depth)
                    })
            })
            .collect::<Vec<_>>()
    }

    fn swap_places(
        &self,
        amphipod: Amphipod,
        hallway_index: usize,
        room_index: usize,
        depth: usize,
    ) -> Option<Self> {
        let steps = HALLWAY_COSTS[room_index][hallway_index];
        if steps == 0 {
            return None;
        }

        let energy = amphipod.energy(depth + steps);

        // swap the hallway spot with the room spot
        let mut amphipods = self.amphipods.clone();
        std::mem::swap(
            &mut amphipods.hallway[hallway_index],
            &mut amphipods.rooms[room_index].1[depth],
        );

        Some(State {
            amphipods,
            energy_spent: self.energy_spent + energy,
        })
    }
}

static HALLWAY_COSTS: [[usize; 11]; 4] = [
    [3, 2, 0, 2, 0, 4, 0, 6, 0, 8, 9], // room A
    [5, 4, 0, 2, 0, 2, 0, 4, 0, 6, 7], // room B
    [7, 6, 0, 4, 0, 2, 0, 2, 0, 4, 5], // room C
    [9, 8, 0, 6, 0, 4, 0, 2, 0, 2, 3], // room D
];

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy_spent.cmp(&self.energy_spent)
    }
}

fn problem(input: &Input) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(State {
        amphipods: input.clone(),
        energy_spent: 0,
    });

    let mut cost_cache = HashMap::new();
    cost_cache.insert(input.clone(), 0);

    while let Some(state) = queue.pop() {
        if state.amphipods.is_done() {
            return state.energy_spent;
        }

        let t = state.all_transitions();

        for next in t {
            if next.energy_spent < *cost_cache.get(&next.amphipods).unwrap_or(&usize::MAX) {
                cost_cache.insert(next.amphipods.clone(), next.energy_spent);
                queue.push(next);
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem(&input);
        assert_eq!(result, 12521)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem(&input);
        assert_eq!(result, 44169)
    }
}
