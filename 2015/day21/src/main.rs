use itertools::Itertools;
use std::{iter::Sum, ops::Add};

use common::{answer, read_input};
use nom::{
    bytes::complete::tag,
    character::complete::{i32 as nom_i32, newline},
    combinator::map,
    sequence::{delimited, preceded},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

#[derive(Debug)]
struct Entity {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

pub const fn div_ceil(lhs: i32, rhs: i32) -> i32 {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if (r > 0 && rhs > 0) || (r < 0 && rhs < 0) {
        d + 1
    } else {
        d
    }
}

impl Entity {
    fn new(hit_points: i32, damage: i32, armor: i32) -> Entity {
        Entity {
            hit_points,
            damage,
            armor,
        }
    }

    fn turns_until_victory(&self, other: &Entity) -> i32 {
        // always do at least one damage
        let dmg = (self.damage - other.armor).max(1);
        div_ceil(other.hit_points, dmg)
    }

    fn beats(&self, other: &Entity) -> bool {
        let player_turns = self.turns_until_victory(other);
        let boss_turns = other.turns_until_victory(self);

        player_turns < boss_turns
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Item<'a> {
    name: &'a str,
    cost: i32,
    damage: i32,
    armor: i32,
}
impl<'a> Item<'a> {
    const fn new(name: &str, cost: i32, damage: i32, armor: i32) -> Item<'_> {
        Item {
            name,
            cost,
            damage,
            armor,
        }
    }
}

impl<'a> Sum for Item<'a> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap()
    }
}

impl<'a> Add for Item<'a> {
    type Output = Item<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        Item {
            name: "Backpack",
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}
const WEAPONS: [Item; 5] = [
    Item::new("Dagger", 8, 4, 0),
    Item::new("Shortsword", 10, 5, 0),
    Item::new("Warhammer", 25, 6, 0),
    Item::new("Longsword", 40, 7, 0),
    Item::new("Greataxe", 74, 8, 0),
];

const ARMOR: [Item; 6] = [
    Item::new("None", 0, 0, 0),
    Item::new("Leather", 13, 0, 1),
    Item::new("Chainmail", 31, 0, 2),
    Item::new("Splintmail", 53, 0, 3),
    Item::new("Bandedmail", 75, 0, 4),
    Item::new("Platemail", 102, 0, 5),
];

const RINGS: [Item; 7] = [
    Item::new("None", 0, 0, 0),
    Item::new("Damage +1", 25, 1, 1),
    Item::new("Damage +2", 50, 2, 0),
    Item::new("Damage +3", 100, 3, 0),
    Item::new("Defense +1", 20, 0, 1),
    Item::new("Defense +2", 40, 0, 2),
    Item::new("Defense +3", 80, 0, 3),
];
type Input = Entity;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        (
            delimited(tag("Hit Points: "), nom_i32, newline),
            delimited(tag("Damage: "), nom_i32, newline),
            preceded(tag("Armor: "), nom_i32),
        ),
        |(hp, dmg, armor)| Entity::new(hp, dmg, armor),
    ).parse(input);

    result.unwrap().1
}

fn get_backpacks<'a>() -> impl Iterator<Item = Item<'a>> {
    [
        WEAPONS.to_vec(),
        ARMOR.to_vec(),
        RINGS.to_vec(),
        RINGS.to_vec(),
    ]
    .into_iter()
    // get the cartesian product to get all combos of items we could have
    .multi_cartesian_product()
    // dedup so we don't have two of the same rings on our hands
    .map(|items| items.into_iter().dedup().sum())
}

fn problem1(boss: &Input) -> i32 {
    get_backpacks()
        .filter_map(|backpack| {
            let player = Entity::new(100, backpack.damage, backpack.armor);
            player.beats(boss).then_some(backpack.cost)
        })
        .min()
        .unwrap()
}

fn problem2(boss: &Input) -> i32 {
    get_backpacks()
        .filter_map(|backpack| {
            let player = Entity::new(100, backpack.damage, backpack.armor);
            boss.beats(&player).then_some(backpack.cost)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 78)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 148)
    }
}
