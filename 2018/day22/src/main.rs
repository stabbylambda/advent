use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use common::{
    extensions::PointExt,
    grid::{Grid, HasNeighbors},
    nom::usize,
};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Cave;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_pair(
            preceded(tag("depth: "), usize),
            newline,
            preceded(tag("target: "), separated_pair(usize, tag(","), usize)),
        ),
        |(depth, target)| Cave::new(depth, target),
    )(input);

    result.unwrap().1
}

#[derive(Clone)]
struct Cave {
    depth: usize,
    target: (usize, usize),
    erosion_cache: HashMap<(usize, usize), usize>,
}

impl Cave {
    fn new(depth: usize, target: (usize, usize)) -> Self {
        Cave {
            depth,
            target,
            erosion_cache: HashMap::new(),
        }
    }

    fn get_erosion_values(&self) -> Vec<usize> {
        self.erosion_cache.values().copied().collect()
    }

    fn geologic_index(&mut self, (x, y): (usize, usize)) -> usize {
        match (x, y) {
            (0, 0) => 0,
            (_, 0) => x * 16807,
            (0, _) => y * 48271,
            _ if x == self.target.0 && y == self.target.1 => 0,
            _ => self.erosion_level((x - 1, y)) * self.erosion_level((x, y - 1)),
        }
    }

    fn erosion_level(&mut self, (x, y): (usize, usize)) -> usize {
        if let Some(erosion) = self.erosion_cache.get(&(x, y)) {
            *erosion
        } else {
            let erosion = (self.geologic_index((x, y)) + self.depth) % 20183;
            self.erosion_cache.insert((x, y), erosion);
            erosion
        }
    }

    fn get_type(&self, (x, y): (usize, usize)) -> RegionType {
        match self.erosion_cache.get(&(x, y)).unwrap() % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }

    fn explore(&mut self, padding: usize) {
        let (tx, ty) = self.target;
        for y in 0..=ty + padding {
            for x in 0..=tx + padding {
                self.erosion_level((x, y));
            }
        }
    }

    fn create_map(&mut self, padding: usize) -> Grid<RegionType> {
        self.explore(padding);

        let (tx, ty) = self.target;
        let width = tx + padding;
        let height = ty + padding;
        let mut v = vec![vec![RegionType::Rocky; width]; height];
        v.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, cell)| {
                *cell = self.get_type((x, y));
            })
        });
        Grid::new(v)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegionType::Rocky => '.',
                RegionType::Wet => '=',
                RegionType::Narrow => '|',
            }
        )
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (tx, ty) = self.target;
        for y in 0..=ty {
            for x in 0..=tx {
                match (x, y) {
                    (0, 0) => write!(f, "M")?,
                    _ if x == tx && y == ty => write!(f, "T")?,
                    _ => write!(f, "{}", self.get_type((x, y)))?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn problem1(input: &Input) -> usize {
    //Explore the cave. This primes the erosion cache, which we need to calculate
    let mut cave = input.clone();
    cave.explore(0);

    cave.get_erosion_values().iter().map(|x| x % 3).sum()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    position: (usize, usize),
    target: (usize, usize),
    distance: usize,
    region_type: RegionType,
    equipped: Tool,
    minutes: usize,
}

impl State {
    fn switch_tool(&self) -> Option<Self> {
        let tool = match (self.region_type, self.equipped) {
            (RegionType::Rocky, Tool::Torch) => Some(Tool::Gear),
            (RegionType::Rocky, Tool::Gear) => Some(Tool::Torch),
            (RegionType::Wet, Tool::Gear) => Some(Tool::None),
            (RegionType::Wet, Tool::None) => Some(Tool::Gear),
            (RegionType::Narrow, Tool::Torch) => Some(Tool::None),
            (RegionType::Narrow, Tool::None) => Some(Tool::Torch),
            _ => None,
        };

        tool.map(|tool| State {
            equipped: tool,
            minutes: self.minutes + 7,
            ..*self
        })
    }

    fn move_to(&self, new_position: (usize, usize), new_region_type: RegionType) -> Option<Self> {
        let can_move = self.equipped.is_valid(new_region_type);

        can_move.then_some(State {
            position: new_position,
            distance: new_position.manhattan(&self.target),
            region_type: new_region_type,
            minutes: self.minutes + 1,
            ..*self
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// implement Ord backwards so the queue is a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then(other.minutes.cmp(&self.minutes))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    Gear,
    None,
}

impl Tool {
    fn is_valid(&self, region: RegionType) -> bool {
        match (region, self) {
            // In rocky regions, you can use the climbing gear or the torch.
            (RegionType::Rocky, Tool::Torch) => true,
            (RegionType::Rocky, Tool::Gear) => true,
            // You cannot use neither (you'll likely slip and fall).
            (RegionType::Rocky, Tool::None) => false,

            // In wet regions, you can use the climbing gear or neither tool.
            (RegionType::Wet, Tool::Gear) => true,
            (RegionType::Wet, Tool::None) => true,
            // You cannot use the torch (if it gets wet, you won't have a light source).
            (RegionType::Wet, Tool::Torch) => false,

            // In narrow regions, you can use the torch or neither tool.
            (RegionType::Narrow, Tool::Torch) => true,
            (RegionType::Narrow, Tool::None) => true,
            // You cannot use the climbing gear (it's too bulky to fit).
            (RegionType::Narrow, Tool::Gear) => false,
        }
    }
}

fn problem2(input: &Input) -> usize {
    let mut cave = input.clone();
    // explore the cave and add some padding because we might need to
    // travel past the target and double back for time
    let map = cave.create_map(5);

    let state = State {
        position: (0, 0),
        target: input.target,
        distance: input.target.manhattan(&(0, 0)),
        region_type: *map.get((0, 0)).data,
        equipped: Tool::Torch,
        minutes: 0,
    };

    let mut seen: HashMap<((usize, usize), Tool), usize> = HashMap::new();
    let mut min_minutes = usize::MAX;
    let mut queue = BinaryHeap::new();
    queue.push(state);

    // let mut states_considered: u64 = 0;
    while let Some(state) = queue.pop() {
        // states_considered += 1;

        // if states_considered % 1_000_000 == 0 {
        //     println!(
        //         "States considered {states_considered}, current distance: {} at {}",
        //         state.distance, state.minutes
        //     );
        // }

        // did we get to the target with the right tool?
        if state.position == input.target && state.equipped == Tool::Torch {
            min_minutes = min_minutes.min(state.minutes);
            continue;
        }

        // If we're already spending more time than the minimum so far, bail
        if state.minutes >= min_minutes {
            continue;
        }

        // if we've been on this square with this tool in less time, bail
        if seen
            .get(&(state.position, state.equipped))
            .unwrap_or(&usize::MAX)
            <= &state.minutes
        {
            continue;
        } else {
            seen.insert((state.position, state.equipped), state.minutes);
        }

        // travel from here or switch tools
        if let Some(switch_tool) = state.switch_tool() {
            queue.push(switch_tool);
        }

        for neighbor in map.neighbors(state.position).to_vec() {
            if let Some(new_position) = state.move_to(neighbor.coords, *neighbor.data) {
                queue.push(new_position);
            }
        }
    }

    min_minutes
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 114)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 45)
    }
}
