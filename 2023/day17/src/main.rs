use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use common::{
    answer,
    grid::{CardinalDirection, Coord, Grid, GridSquare},
    nom::{parse_grid, single_digit},
    read_input,
};
use nom::{IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(single_digit).parse(input);

    result.unwrap().1
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    heat_loss: u32,
    current: Coord,
    consecutive_steps: u32,
    direction: CardinalDirection,
}

impl State {
    fn new(square: GridSquare<u32>, direction: CardinalDirection, consecutive_steps: u32) -> Self {
        Self {
            current: square.coords,
            heat_loss: *square.data,
            direction,
            consecutive_steps,
        }
    }

    fn get_eligible_directions(&self, min: u32, max: u32) -> Vec<CardinalDirection> {
        use CardinalDirection::*;
        match self.direction {
            // if we're less than min, keep going
            x if self.consecutive_steps < min => vec![x],

            // if we're at max, turn left or right
            North | South if self.consecutive_steps == max => vec![East, West],
            East | West if self.consecutive_steps == max => vec![North, South],

            // otherwise we can go anywhere but where we just came from
            North => vec![North, East, West],
            South => vec![South, East, West],
            East => vec![East, North, South],
            West => vec![West, North, South],
        }
    }

    fn to_cache_key(&self) -> CacheKey {
        // because we can't turn around, we can cut the cache key space in half by only considering vertical or horizontal directions
        let dt = match self.direction {
            CardinalDirection::North | CardinalDirection::South => DirectionType::Vertical,
            CardinalDirection::East | CardinalDirection::West => DirectionType::Horizontal,
        };
        (self.current, dt, self.consecutive_steps)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum DirectionType {
    Horizontal,
    Vertical,
}

type CacheKey = ((usize, usize), DirectionType, u32);

fn problem1(input: &Input) -> u32 {
    dijkstra(input, 1, 3)
}

fn problem2(input: &Input) -> u32 {
    dijkstra(input, 4, 10)
}

fn dijkstra(input: &Input, min: u32, max: u32) -> u32 {
    let goal = (input.width - 1, input.height - 1);

    let mut seen: BTreeMap<CacheKey, u32> = BTreeMap::new();
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();

    // start by going south and east
    let initial_east = State::new(input.get((1, 0)), CardinalDirection::East, 1);
    let initial_south = State::new(input.get((0, 1)), CardinalDirection::South, 1);

    seen.insert(initial_east.to_cache_key(), 0);
    queue.push(Reverse(initial_east));
    seen.insert(initial_south.to_cache_key(), 0);
    queue.push(Reverse(initial_south));

    while let Some(Reverse(state)) = queue.pop() {
        // if we're at the goal, mark this as best available
        if state.current == goal {
            return state.heat_loss;
        }

        let current = input.get(state.current);
        let neighbors = current.neighbors();

        // get all the eligible neighbors
        for dir in state.get_eligible_directions(min, max) {
            if let Some(neighbor) = neighbors.get(dir) {
                let heat_loss = state.heat_loss + *neighbor.data;

                let consecutive_steps = if dir == state.direction {
                    state.consecutive_steps + 1
                } else {
                    1
                };

                let next = State {
                    current: neighbor.coords,
                    heat_loss,
                    consecutive_steps,
                    direction: dir,
                };

                // figure out if we should go to the next state
                let prev_cost = seen.get(&next.to_cache_key()).unwrap_or(&u32::MAX);
                if next.heat_loss < *prev_cost {
                    seen.insert(next.to_cache_key(), heat_loss);
                    queue.push(Reverse(next));
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 102)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 94)
    }
}
