use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};
use rayon::prelude::*;

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Blueprint>;
type Costs = [[u32; 4]; 4];
#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: [[u32; 4]; 4],
    max_costs: [u32; 4],
}

impl Blueprint {
    fn new(id: u32, costs: Costs) -> Blueprint {
        Blueprint {
            id,
            costs,
            max_costs: Blueprint::max_items_needed(&costs),
        }
    }
    fn max_items_needed(costs: &Costs) -> [u32; 4] {
        let mut result = [0, 0, 0, u32::MAX];

        for cost in costs {
            for (item, &amount) in cost.iter().enumerate() {
                result[item] = result[item].max(amount);
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy)]
struct Inventory {
    items: [u32; 4],
    bots: [u32; 4],
    time_left: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Types {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl Inventory {
    fn new(time_left: u32) -> Inventory {
        Inventory {
            items: [0, 0, 0, 0],
            bots: [1, 0, 0, 0],
            time_left,
        }
    }

    fn gather(&self) -> Inventory {
        let new_items = (0..4)
            .map(|n| self.items[n] + self.bots[n])
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        Inventory {
            items: new_items,
            bots: self.bots,
            time_left: self.time_left - 1,
        }
    }

    fn can_build(&self, bp: &Blueprint, robot_type: Types) -> bool {
        let robot_type = robot_type as usize;
        let need_more = self.bots[robot_type] < bp.max_costs[robot_type];

        need_more
            && bp.costs[robot_type]
                .iter()
                .zip(self.items)
                .all(|(&cost, items)| cost <= items)
    }

    fn items_remaining(&self, robot_type: Types) -> u32 {
        let idx = robot_type as usize;
        let item_count = self.items[idx];
        let bot_collection_count = self.bots[idx] * self.time_left;

        item_count + bot_collection_count
    }

    fn build_robot(&mut self, bp: &Blueprint, robot_type: Types) {
        self.bots[robot_type as usize] += 1;

        for idx in 0..4 {
            self.items[idx] -= bp.costs[robot_type as usize][idx];
        }
    }

    fn best_possible(&self, robot_type: Types) -> u32 {
        let items_remaining = self.items_remaining(robot_type);
        let bots_to_be_added = self.time_left * (self.time_left - 1) / 2;

        items_remaining + bots_to_be_added
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            (
                (delimited(tag("Blueprint "), nom_u32, tag(": "))),
                (delimited(tag("Each ore robot costs "), nom_u32, tag(" ore. "))),
                (delimited(tag("Each clay robot costs "), nom_u32, tag(" ore. "))),
                ((
                    delimited(tag("Each obsidian robot costs "), nom_u32, tag(" ore ")),
                    delimited(tag("and "), nom_u32, tag(" clay. ")),
                )),
                ((
                    delimited(tag("Each geode robot costs "), nom_u32, tag(" ore ")),
                    delimited(tag("and "), nom_u32, tag(" obsidian.")),
                )),
            ),
            |(id, ore, clay, (obs_ore, obs_clay), (geode_ore, geode_obs))| {
                Blueprint::new(
                    id,
                    [
                        [ore, 0, 0, 0],
                        [clay, 0, 0, 0],
                        [obs_ore, obs_clay, 0, 0],
                        [geode_ore, 0, geode_obs, 0],
                    ],
                )
            },
        ),
    ).parse(input);

    result.unwrap().1
}

fn simulate(
    inventory: Inventory,
    bp: &Blueprint,
    best_so_far: u32,
    previous_skip: &[Types],
) -> u32 {
    // If there's no more time left, or we can't possibly create another geode bot, we need to bail
    let needed_obsidian = bp.max_costs[Types::Obsidian as usize];
    if inventory.time_left == 1 || inventory.best_possible(Types::Obsidian) < needed_obsidian {
        return inventory.items_remaining(Types::Geode);
    }

    // this branch is trash, just get out of it
    if inventory.best_possible(Types::Geode) < best_so_far {
        return 0;
    }

    // if we can build geode, this is the only logical path, ignore the rest of the robot types
    if inventory.can_build(bp, Types::Geode) {
        let mut new_inventory = inventory.gather();
        new_inventory.build_robot(bp, Types::Geode);
        return simulate(new_inventory, bp, best_so_far, &[]);
    }

    let remaining = [Types::Ore, Types::Clay, Types::Obsidian];
    let can_build: Vec<Types> = remaining
        .into_iter()
        .filter(|t| inventory.can_build(bp, *t))
        .filter(|t| !previous_skip.contains(t)) // prune branches where we tried to
        .collect();

    let best = can_build
        .par_iter()
        .map(|robot_type| {
            let mut new_inventory = inventory.gather();
            new_inventory.build_robot(bp, *robot_type);
            simulate(new_inventory, bp, best_so_far, &[])
        })
        .max()
        .unwrap_or(best_so_far)
        .max(best_so_far);

    // Worst case scenario, just gather items
    simulate(inventory.gather(), bp, best, &can_build[..]).max(best)
}

fn problem1(input: &Input) -> u32 {
    let max_time = 24;
    input
        .par_iter()
        .map(|bp| {
            let i = Inventory::new(max_time);
            bp.id * simulate(i, bp, 0, &[])
        })
        .sum()
}

fn problem2(input: &Input) -> u32 {
    let max_time = 32;
    input
        .par_iter()
        .take(3)
        .map(|bp| {
            let i = Inventory::new(max_time);
            simulate(i, bp, 0, &[])
        })
        .product()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 33)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 56 * 62)
    }
}
