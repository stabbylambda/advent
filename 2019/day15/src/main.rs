use std::collections::{BinaryHeap, HashMap, HashSet};

use common::{answer, grid::Grid, read_input};
use intcode::Intcode;
use itertools::{Itertools, MinMaxResult};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

fn main() {
    let input = read_input!();
    let input = Intcode::parse(input);

    let (result1, map) = problem1(&input);
    answer!(result1);
    answer!(problem2(map));
}

type Input = Intcode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
enum BotDirections {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Robot {
    position: (i64, i64),
    program: Intcode,
    steps: u32,
    last_output: Option<((i64, i64), MoveResult)>,
}

impl Robot {
    fn new(input: &Intcode) -> Self {
        Robot {
            position: (0, 0),
            program: input.clone(),
            steps: 0,
            last_output: None,
        }
    }

    fn execute(&mut self, command: BotDirections) {
        // keep the output clear since we don't need to just let it grow infinitely
        self.program.output.clear();
        self.program.input.push(command as i64);
        self.steps += 1;
        self.program.execute();

        let move_result = FromPrimitive::from_i64(self.program.get_last_output());
        let (x, y) = self.position;
        let new_position = match command {
            BotDirections::North => (x, y - 1),
            BotDirections::South => (x, y + 1),
            BotDirections::West => (x - 1, y),
            BotDirections::East => (x + 1, y),
        };

        let Some(move_result) = move_result else {
            return;
        };
        self.last_output = Some((new_position, move_result));
        if let MoveResult::FoundGenerator | MoveResult::Moved = move_result {
            self.position = new_position;
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
enum MoveResult {
    HitWall = 0,
    Moved = 1,
    FoundGenerator = 2,
}
impl PartialOrd for Robot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Robot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Space,
    Generator,
    Nothing,
}

struct TileMap(HashMap<(i64, i64), Tile>);

fn problem1(input: &Input) -> (u32, TileMap) {
    let robot = Robot::new(input);

    let mut moves: HashSet<(i64, i64, BotDirections)> = HashSet::new();
    let mut robot_steps = 0;
    let mut map: TileMap = TileMap(HashMap::new());

    let mut queue = BinaryHeap::new();
    queue.push(robot);

    while let Some(robot) = queue.pop() {
        if let Some((position, move_result)) = robot.last_output {
            match move_result {
                MoveResult::Moved => {
                    map.0.insert(position, Tile::Space);
                }
                MoveResult::FoundGenerator => {
                    map.0.insert(position, Tile::Generator);
                    // we can't return here anymore because we actually have to explore the entire stupid map
                    robot_steps = robot.steps;
                }
                MoveResult::HitWall => {
                    map.0.insert(position, Tile::Wall);
                }
            }
        }

        for command in [
            BotDirections::North,
            BotDirections::South,
            BotDirections::East,
            BotDirections::West,
        ] {
            // don't do the same thing from the same place again
            if moves.insert((robot.position.0, robot.position.1, command)) {
                let mut new_robot = robot.clone();
                new_robot.execute(command);
                queue.push(new_robot);
            }
        }
    }

    (robot_steps, map)
}

fn problem2(map: TileMap) -> u32 {
    // create a map from the HashMap of points to Tiles
    let MinMaxResult::MinMax(x_min, x_max) = map.0.keys().map(|x| x.0).minmax() else {
        panic!()
    };
    let MinMaxResult::MinMax(y_min, y_max) = map.0.keys().map(|x| x.1).minmax() else {
        panic!()
    };
    let map: Grid<Tile> = Grid::new(
        (y_min..=y_max)
            .map(|y| {
                (x_min..=x_max)
                    .map(|x| map.0.get(&(x, y)).copied().unwrap_or(Tile::Nothing))
                    .collect_vec()
            })
            .collect_vec(),
    );

    // find the coordinates of the generator
    let start = map
        .iter()
        .find_map(|x| (*x.data == Tile::Generator).then_some(x.coords))
        .unwrap();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start);

    let mut queue = BinaryHeap::new();
    queue.push((0, start));

    let mut max_length = u32::MIN;

    while let Some((depth, current)) = queue.pop() {
        let neighbors = map.get(current).neighbors();
        let adjacent_spaces = neighbors
            .iter()
            .filter(|x| *x.data == Tile::Space)
            // insert each into the map and only keep the ones we haven't been to yet
            .filter(|x| visited.insert(x.coords))
            .collect_vec();

        if adjacent_spaces.is_empty() {
            // we hit a dead end, so figure out if this is the longest path
            max_length = max_length.max(depth);
        }

        for neighbor in adjacent_spaces {
            queue.push((depth + 1, neighbor.coords));
        }
    }

    max_length
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::{problem1, problem2};
    #[test]
    #[ignore = "input files aren't available in CI"]
    fn first() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let (result, map) = problem1(&input);
        assert_eq!(result, 214);
        let result = problem2(map);
        assert_eq!(result, 344);
    }
}
