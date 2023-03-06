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
    // btrees keep everything in order...so we'll always get "reading order" for free
    spaces: BTreeSet<Point>,
    units: BTreeMap<usize, Unit>,
    rounds: u32,
    height: usize,
    width: usize,
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
    fn new(grid: Vec<Vec<char>>) -> Self {
        let units: BTreeMap<usize, Unit> = grid
            .iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.iter().enumerate().filter_map(move |(x, t)| match t {
                    'E' => Some(((y, x), UnitType::Elf)),
                    'G' => Some(((y, x), UnitType::Goblin)),
                    _ => None,
                })
            })
            .enumerate()
            .map(|(id, (loc, side))| (id, Unit::new(id, loc, side)))
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

    fn remaining_hp(&self) -> u32 {
        self.units.values().map(|u| u.health).sum()
    }

    fn outcome(&self) -> u32 {
        println!("Done at round {} with {}", self.rounds, self.remaining_hp());
        self.rounds * self.remaining_hp()
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

    fn get_potential_locations(&self, id: usize) -> Vec<Point> {
        let unit = self.get_unit_by_id(id).unwrap();
        let location = unit.location;

        let mut potential_locations: Vec<Point> = self
            .units
            .values()
            .filter_map(|u| (u.location != location && u.side != unit.side).then_some(u.location))
            .flat_map(|l| self.neighbors(l))
            .collect();
        potential_locations.sort();
        potential_locations
    }

    fn is_open(&self, location: Point) -> bool {
        self.get_unit_at_location(location).is_none() && self.spaces.contains(&location)
    }

    fn get_next_step(&self, id: usize) -> Option<Point> {
        let location = self.get_unit_by_id(id).unwrap().location;
        let enemy_adjacent = self.get_potential_locations(id);
        let mut visited: BTreeSet<Point> = BTreeSet::new();
        let mut paths: VecDeque<Vec<Point>> = VecDeque::new();
        let start: Vec<Vec<Point>> = self
            .neighbors(location)
            .iter()
            .filter(|x| self.is_open(**x))
            .map(|p| vec![*p])
            .collect();
        paths.extend(start);

        while let Some(path) = paths.pop_front() {
            // get the last point in this path
            let furthest = path.last().unwrap();

            // we found it! return the first step we need to take
            if enemy_adjacent.contains(furthest) {
                return path.first().cloned();
            }

            // if we haven't been here before, we need to find new paths from here
            if !visited.contains(furthest) {
                visited.insert(*furthest);
                for neighbor in self
                    .neighbors(*furthest)
                    .into_iter()
                    // we haven't already seen this
                    .filter(|x| !visited.contains(x))
                    // there isn't someone currently there
                    .filter(|x| self.is_open(*x))
                {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);

                    paths.push_back(new_path);
                }
            }
        }

        None
    }

    fn get_adjacent_enemy(&self, id: usize) -> Option<&Unit> {
        let unit = self.get_unit_by_id(id)?;

        self.neighbors(unit.location)
            .into_iter()
            .filter_map(|n| self.get_unit_at_location(n))
            .filter(|u| u.side != unit.side)
            .min_by_key(|u| (u.health, u.location))
    }

    fn attack(&mut self, attacker_id: usize, enemy_id: usize) {
        let damage = self
            .get_unit_by_id(attacker_id)
            .map(|x| x.power)
            .unwrap_or_default();

        if let Some(victim) = self.units.get_mut(&enemy_id) {
            victim.take_damage(damage);
            if victim.is_dead() {
                self.units.remove(&enemy_id);
            }
        };
    }

    fn move_unit(&mut self, id: usize, next: (usize, usize)) {
        if let Some(unit) = self.units.get_mut(&id) {
            // println!("Moving {unit} from {location:?} -> {next:?}");
            unit.location = next;
        }
    }

    fn run(&mut self) -> u32 {
        while !self.is_over() {
            // println!("=========== Round {} ===========", self.rounds);
            for id in self.unit_ids_in_reading_order() {
                // if this unit already died in a previous attack this round, we're done
                if self.get_unit_by_id(id).is_none() {
                    continue;
                };

                // if we don't have any enemies, we're done
                if !self.unit_has_enemies(id) {
                    return self.outcome();
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
                    self.attack(id, enemy_id);
                }
            }

            self.rounds += 1;
            // println!("{self}");
        }
        self.outcome()
    }
}

fn problem1(input: &Input) -> u32 {
    let mut game = Game::new(input.clone());
    game.run()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
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
    fn actual_input() {
        let input = include_str!("../input.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 201856);
    }

    #[test]
    fn examples() {
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
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
