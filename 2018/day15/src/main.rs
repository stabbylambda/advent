use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Display,
};

use advent_2018_15::{reading_neighbors, Point, Unit, UnitType};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input.lines().map(|x| x.chars().collect()).collect()
}

struct Game {
    spaces: BTreeSet<Point>,
    units: BTreeMap<usize, Unit>,
    rounds: u32,
    height: usize,
    width: usize,
}

struct GameOutcome {
    rounds: u32,
    units: Vec<Unit>,
}

impl GameOutcome {
    fn score(&self) -> u32 {
        self.rounds * self.remaining_hp()
    }

    fn remaining_hp(&self) -> u32 {
        self.units.iter().map(|u| u.health).sum()
    }

    fn count_units(&self, side: UnitType) -> usize {
        self.units.iter().filter(|u| u.side == side).count()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(side) = self.get_unit_at_location((y, x)).map(|x| x.side) {
                    match side {
                        UnitType::Elf => write!(f, "E")?,
                        _ => write!(f, "G")?,
                    };
                } else if self.spaces.contains(&(y, x)) {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }

            let units = self
                .units
                .values()
                .filter_map(|unit| (unit.location.0 == y).then_some(unit))
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, " {units}")?;

            writeln!(f)?;
        }
        Ok(())
    }
}

impl Game {
    fn new(grid: Vec<Vec<char>>, elf_power: u32) -> Self {
        let units: BTreeMap<usize, Unit> = grid
            .iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.iter().enumerate().filter_map(move |(x, t)| match t {
                    'E' => Some(((y, x), UnitType::Elf, elf_power)),
                    'G' => Some(((y, x), UnitType::Goblin, 3)),
                    _ => None,
                })
            })
            .enumerate()
            .map(|(id, (loc, side, power))| (id, Unit::new(id, loc, side, power)))
            .collect();

        let spaces: BTreeSet<(usize, usize)> = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, t)| match t {
                    '#' => None,
                    _ => Some((y, x)),
                })
            })
            .collect();

        Game {
            units,
            spaces,
            rounds: 0,
            height: grid.len(),
            width: grid.first().unwrap().len(),
        }
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
        // only return valid spaces in "reading" order
        reading_neighbors(point)
            .iter()
            .filter_map(|x| x.filter(|p| self.spaces.contains(p)))
            .collect()
    }

    fn unit_ids_in_reading_order(&self) -> Vec<usize> {
        let mut units: Vec<&Unit> = self.units.values().collect();
        units.sort_unstable_by_key(|x| x.location);
        units.iter().map(|x| x.id).collect()
    }

    fn outcome(&self) -> GameOutcome {
        GameOutcome {
            rounds: self.rounds,
            units: self.units.values().cloned().collect(),
        }
    }

    fn is_over(&self) -> bool {
        let goblins = self.units.values().any(|u| u.side == UnitType::Goblin);
        let elves = self.units.values().any(|u| u.side == UnitType::Elf);

        goblins != elves
    }

    fn unit_has_enemies(&self, id: usize) -> bool {
        let side = self.get_unit_by_id(id).map(|x| x.side).unwrap();
        self.units.values().any(|u| u.side != side)
    }

    fn get_unit_by_id(&self, id: usize) -> Option<&Unit> {
        self.units.get(&id)
    }

    fn get_unit_at_location(&self, point: Point) -> Option<&Unit> {
        self.units.values().find(|u| u.location == point)
    }

    fn is_open(&self, location: Point) -> bool {
        self.get_unit_at_location(location).is_none() && self.spaces.contains(&location)
    }

    fn get_next_step(&self, id: usize) -> Option<Point> {
        let unit = self.get_unit_by_id(id).unwrap();
        // find all the potential targets (locations next to enemy units that are open)
        let targets: Vec<(usize, usize)> = self
            .units
            .values()
            .filter(|u| u.location != unit.location && u.side != unit.side)
            .flat_map(|u| self.neighbors(u.location))
            .filter(|l| self.is_open(*l))
            .collect();

        // Seed the queue with paths that include the start point of each of our valid neighbors
        let mut paths: VecDeque<Vec<Point>> = VecDeque::new();
        let start: Vec<Vec<Point>> = self
            .neighbors(unit.location)
            .iter()
            .filter(|x| self.is_open(**x))
            .map(|p| vec![*p])
            .collect();
        paths.extend(start);

        // Keep track of all the places we've been, paths we've seen, plus the length of
        // the shortest path we've found so far
        let mut visited: BTreeSet<Point> = BTreeSet::new();
        let mut min_path = usize::MAX;
        let mut all_paths = vec![];

        // bfs
        while let Some(path) = paths.pop_front() {
            // get the last point in this path
            let furthest = path.last().unwrap();

            // we found it! return the first step we need to take
            if targets.contains(furthest) {
                min_path = path.len();
                all_paths.push(path.clone());
                continue;
            }

            // is this longer than the shortest full path we've found? if so, bail
            if path.len() > min_path {
                continue;
            }

            // if we haven't been here before, we need to find new paths from here
            if visited.insert(*furthest) {
                for neighbor in self
                    .neighbors(*furthest)
                    .into_iter()
                    // we haven't already seen this and someone isn't there already
                    .filter(|x| !visited.contains(x) && self.is_open(*x))
                {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);

                    paths.push_back(new_path);
                }
            }
        }

        all_paths
            .iter()
            // pick the shortest, most reading-order endpoint
            .min_by_key(|p| (p.len(), p.last().unwrap()))
            // then get the first step on that path
            .and_then(|x| x.first())
            .cloned()
    }

    fn get_adjacent_enemy(&self, id: usize) -> Option<&Unit> {
        let unit = self.get_unit_by_id(id)?;

        self.neighbors(unit.location)
            .into_iter()
            .filter_map(|n| self.get_unit_at_location(n))
            .filter(|u| u.side != unit.side)
            // sort by health and then reading order
            .min_by_key(|u| (u.health, u.location))
    }

    fn attack(&mut self, attacker_id: usize, enemy_id: usize) -> Option<UnitType> {
        let damage = self
            .get_unit_by_id(attacker_id)
            .map(|x| x.power)
            .unwrap_or_default();

        if let Some(victim) = self.units.get_mut(&enemy_id) {
            victim.take_damage(damage);
            if victim.is_dead() {
                let side = victim.side;
                self.units.remove(&enemy_id);
                return Some(side);
            }
        };

        None
    }

    fn move_unit(&mut self, id: usize, next: (usize, usize)) {
        if let Some(unit) = self.units.get_mut(&id) {
            unit.location = next;
        }
    }

    fn run(&mut self, terminate_on_elf_death: bool) -> Option<GameOutcome> {
        while !self.is_over() {
            for id in self.unit_ids_in_reading_order() {
                // if this unit already died in a previous attack this round, we're done
                if self.get_unit_by_id(id).is_none() {
                    continue;
                };

                // if we don't have any enemies, we're done
                if !self.unit_has_enemies(id) {
                    return Some(self.outcome());
                }

                // if we're not standing next to an enemy, move
                if self.get_adjacent_enemy(id).is_none() {
                    // can we move?
                    if let Some(next) = self.get_next_step(id) {
                        self.move_unit(id, next);
                    }
                }

                // attack an enemy if we're standing next to them
                if let Some(enemy_id) = self.get_adjacent_enemy(id).map(|x| x.id) {
                    if let Some(dead_unit_type) = self.attack(id, enemy_id) {
                        // early terminate if we just killed an elf
                        if dead_unit_type == UnitType::Elf && terminate_on_elf_death {
                            return None;
                        }
                    }
                }
            }

            self.rounds += 1;
        }
        Some(self.outcome())
    }
}

fn problem1(input: &Input) -> u32 {
    let mut game = Game::new(input.clone(), 3);
    game.run(false).unwrap().score()
}

fn problem2(input: &Input) -> u32 {
    // figure out how many elves there are in the input
    let elf_count = input.iter().flatten().filter(|c| **c == 'E').count();

    for power in 4.. {
        // create a new game with the given elf power
        let mut game = Game::new(input.clone(), power);
        if let Some(outcome) = game.run(true) {
            let elves_left = outcome.count_units(UnitType::Elf);

            if elves_left == elf_count {
                return outcome.score();
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
        assert_eq!(result, 27730);
    }

    #[test]
    fn examples_part1() {
        let cases = [
            (include_str!("../examples/36334.txt"), 36334),
            (include_str!("../examples/39514.txt"), 39514),
            (include_str!("../examples/27755.txt"), 27755),
            (include_str!("../examples/28944.txt"), 28944),
            (include_str!("../examples/18740.txt"), 18740),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem1(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn examples_part2() {
        let cases = [
            (include_str!("../examples/36334.txt"), 29064),
            (include_str!("../examples/39514.txt"), 31284),
            (include_str!("../examples/27755.txt"), 3478),
            (include_str!("../examples/28944.txt"), 6474),
            (include_str!("../examples/18740.txt"), 1140),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem2(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4988);
    }

    #[test]
    fn actual_input() {
        let input = include_str!("../input.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 48034);
    }
}
