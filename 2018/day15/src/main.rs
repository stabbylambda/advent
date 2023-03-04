use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Display,
};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Clone, Copy, Debug)]
struct Unit {
    health: u32,
    power: u32,
    side: UnitType,
}

impl Unit {
    fn new(side: UnitType) -> Self {
        Unit {
            health: 200,
            power: 3,
            side,
        }
    }

    fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage)
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }
}

type Point = (usize, usize);

struct Game {
    // btrees keep everything in order...so we'll always get "reading order" for free
    spaces: BTreeSet<Point>,
    units: BTreeMap<Point, Unit>,
    rounds: u32,
    height: usize,
    width: usize,
}

impl Game {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let units: BTreeMap<Point, Unit> = grid
            .iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.iter().enumerate().filter_map(move |(x, t)| match t {
                    'E' => Some(((y, x), Unit::new(UnitType::Elf))),
                    'G' => Some(((y, x), Unit::new(UnitType::Goblin))),
                    _ => None,
                })
            })
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

    fn neighbors(&self, (y, x): (usize, usize)) -> Vec<Point> {
        let north = (y != 0).then_some((y - 1, x));
        let east = Some((y, x + 1));
        let west = (x != 0).then_some((y, x - 1));
        let south = Some((y + 1, x));

        // only return valid spaces in "reading" order
        vec![north, west, east, south]
            .iter()
            .filter_map(|x| x.filter(|p| self.spaces.contains(p)))
            .collect()
    }

    fn unit_locations(&self) -> Vec<Point> {
        self.units.keys().cloned().collect()
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

    fn unit_has_enemies(&self, unit: Point) -> bool {
        let side = self.units.get(&unit).unwrap().side;
        self.units.values().any(|u| u.side != side)
    }

    fn remove_unit(&mut self, location: Point) {
        self.units.remove(&location);
    }

    fn get_unit(&self, location: Point) -> Option<&Unit> {
        self.units.get(&location)
    }

    fn get_potential_locations(&self, location: Point) -> Vec<Point> {
        let side = self.get_unit(location).map(|x| x.side).unwrap();
        let mut potential_locations: Vec<Point> = self
            .units
            .iter()
            .filter_map(|(l, u)| (*l != location && u.side != side).then_some(l))
            .flat_map(|l| self.neighbors(*l))
            .collect();
        potential_locations.sort();
        potential_locations
    }

    fn is_open(&self, location: Point) -> bool {
        !self.units.contains_key(&location) && self.spaces.contains(&location)
    }

    fn get_next_step(&self, location: Point) -> Option<Point> {
        let enemy_adjacent = self.get_potential_locations(location);
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

    fn get_adjacent_enemy(&self, location: Point) -> Option<Point> {
        let unit = self.units[&location];
        self.neighbors(location)
            .into_iter()
            .filter_map(|n| self.get_unit(n).map(|u| (n, u)))
            .filter(|(_l, u)| u.side != unit.side)
            .min_by_key(|(loc, u)| (u.health, *loc))
            .map(|(l, _u)| l)
    }

    fn attack(&mut self, attacker_location: Point, victim_location: Point) {
        let damage = self
            .units
            .get(&attacker_location)
            .map(|x| x.power)
            .unwrap_or_default();

        if let Some(victim) = self.units.get_mut(&victim_location) {
            victim.take_damage(damage);
            if victim.is_dead() {
                self.remove_unit(victim_location);
            }
        };
    }

    fn move_unit(&mut self, location: (usize, usize), next: (usize, usize)) {
        let unit = self.units.remove(&location).unwrap();
        self.units.insert(next, unit);
    }
}

fn problem1(input: &Input) -> u32 {
    let mut game = Game::new(input.clone());

    while !game.is_over() {
        println!("=========== Round {} ===========", game.rounds);

        // the basic sketch of what this loop looks like:
        for location in game.unit_locations() {
            let mut location = location;

            // if this unit already died in a previous attack this round, we're done
            if game.get_unit(location).is_none() {
                continue;
            };

            // if we don't have any enemies, we're done
            if !game.unit_has_enemies(location) {
                return game.outcome();
            }

            // if we're not standing next to an enemy, move
            if game.get_adjacent_enemy(location).is_none() {
                if let Some(next) = game.get_next_step(location) {
                    game.move_unit(location, next);
                    location = next;
                }
            }

            // attack an enemy if we're standing next to them
            if let Some(enemy_location) = game.get_adjacent_enemy(location) {
                game.attack(location, enemy_location);
            }
        }

        game.rounds += 1;
        println!("{game}");
    }

    game.outcome()
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(unit) = self.get_unit((y, x)) {
                    match unit.side {
                        UnitType::Elf => write!(f, "E")?,
                        _ => write!(f, "G")?,
                    };
                } else if self.spaces.contains(&(y, x)) {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }

            for unit in self.units.iter().filter(|(loc, unit)| loc.0 == y) {
                write!(f, " {:?}({}) ", unit.1.side, unit.1.health)?;
            }

            writeln!(f)?;
        }
        Ok(())
    }
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
        assert_eq!(result, 27730)
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
