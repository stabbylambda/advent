use common::{
    answer,
    grid::{CardinalDirection, Coord, Grid},
    nom::parse_grid,
    read_input,
};
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Grid<Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Space,
    Wall,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = parse_grid(alt((
        map(char('S'), |_| Tile::Start),
        map(char('E'), |_| Tile::End),
        map(char('.'), |_| Tile::Space),
        map(char('#'), |_| Tile::Wall),
    ))).parse(input);

    result.unwrap().1
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    position: Coord,
    facing: CardinalDirection,
    tiles: Vec<Coord>,
}
impl State {
    fn new(position: Coord) -> Self {
        Self {
            cost: 0,
            position,
            facing: CardinalDirection::East,
            tiles: vec![position],
        }
    }
    fn moves(&self, grid: &Grid<Tile>) -> Vec<State> {
        let mut valid = vec![];
        valid.push(Self {
            facing: self.facing.turn_left(),
            cost: self.cost + 1000,
            tiles: self.tiles.clone(),
            ..*self
        });

        valid.push(Self {
            facing: self.facing.turn_right(),
            cost: self.cost + 1000,
            tiles: self.tiles.clone(),
            ..*self
        });

        if let Some(forward) = grid
            .get_neighbor(self.position, self.facing)
            .filter(|x| matches!(x.data, Tile::Space | Tile::End | Tile::Start))
            .filter(|x| !self.tiles.contains(&x.coords))
        {
            let mut new_tiles = self.tiles.clone();
            new_tiles.push(forward.coords);
            valid.push(Self {
                position: forward.coords,
                cost: self.cost + 1,
                tiles: new_tiles,
                ..*self
            });
        }

        valid
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn solve(input: &Input) -> State {
    let position = input.find(Tile::Start).map(|x| x.coords).unwrap();

    let mut seen: BTreeMap<(Coord, CardinalDirection), usize> = BTreeMap::new();
    let mut queue = BinaryHeap::from_iter([State::new(position)]);
    let mut best: Option<State> = None;
    while let Some(current) = queue.pop() {
        let square = input.get(current.position);

        // don't continue if we're already over a previous best
        if let Some(previous_best) = &best {
            if current.cost > previous_best.cost {
                break;
            }
        }

        // have we already been here?
        let previous = seen
            .entry((current.position, current.facing))
            .or_insert(usize::MAX);

        if *previous < current.cost {
            continue;
        } else {
            *previous = current.cost;
        }

        // did we hit an end?
        if square.data == &Tile::End {
            if let Some(previous_best) = &mut best {
                if previous_best.cost == current.cost {
                    previous_best.tiles.extend(&current.tiles);
                    continue;
                } else {
                    continue;
                }
            } else {
                best = Some(current.clone());
            }
        }

        // we're not there yet, keep looking
        for valid_move in &current.moves(input) {
            queue.push(valid_move.clone());
        }
    }

    best.unwrap()
}

fn problem1(input: &Input) -> usize {
    solve(input).cost
}

fn problem2(input: &Input) -> usize {
    solve(input)
        .tiles
        .into_iter()
        .collect::<BTreeSet<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first_small() {
        let input = include_str!("../test_small.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 7036)
    }

    #[test]
    fn first_large() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 11048)
    }
    #[test]
    fn second_small() {
        let input = include_str!("../test_small.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 45)
    }

    #[test]
    fn second_large() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 64)
    }
}
