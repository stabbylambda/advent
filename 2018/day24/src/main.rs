use itertools::Itertools;
use std::{cmp::Reverse, collections::HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Group>;

fn parse(input: &str) -> Input {
    let damage_type = |s| {
        alt((
            map(tag("fire"), |_| DamageType::Fire),
            map(tag("cold"), |_| DamageType::Cold),
            map(tag("radiation"), |_| DamageType::Radiation),
            map(tag("bludgeoning"), |_| DamageType::Bludgeoning),
            map(tag("slashing"), |_| DamageType::Slashing),
        )).parse(s)
    };

    let damage_modifiers = |s| {
        map(
            delimited(
                tag("("),
                many1(alt((
                    preceded(
                        tag("weak to "),
                        terminated(
                            separated_list1(tag(", "), map(damage_type, DamageModifier::weak)),
                            opt(tag("; ")),
                        ),
                    ),
                    preceded(
                        tag("immune to "),
                        terminated(
                            separated_list1(tag(", "), map(damage_type, DamageModifier::immune)),
                            opt(tag("; ")),
                        ),
                    ),
                ))),
                tag(") "),
            ),
            |modifiers| {
                modifiers
                    .into_iter()
                    .flatten()
                    .collect::<Vec<DamageModifier>>()
            },
        ).parse(s)
    };
    let attack = |s| {
        map(
            separated_pair(u32, tag(" "), damage_type),
            |(amount, attack_type)| Attack {
                amount,
                attack_type,
            },
        ).parse(s)
    };
    let group = |side: Side| {
        move |s| {
            map(
                (
                    terminated(u32, tag(" units each with ")),
                    terminated(u32, tag(" hit points ")),
                    opt(damage_modifiers),
                    delimited(tag("with an attack that does "), attack, tag(" damage")),
                    preceded(tag(" at initiative "), u32),
                ),
                |(units, hit_points, modifiers, attack, initiative)| Group {
                    side,
                    units,
                    hit_points,
                    modifiers: modifiers.unwrap_or_default(),
                    attack,
                    initiative,
                },
            ).parse(s)
        }
    };
    let groups = |side| move |s| separated_list1(newline, group(side)).parse(s);

    let result: IResult<&str, Input> = map(
        separated_pair(
            preceded(tag("Immune System:\n"), groups(Side::ImmuneSystem)),
            tag("\n\n"),
            preceded(tag("Infection:\n"), groups(Side::Infection)),
        ),
        |(immune, infection)| vec![immune, infection].into_iter().flatten().collect(),
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DamageType {
    Fire,
    Cold,
    Radiation,
    Bludgeoning,
    Slashing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ModifierType {
    Weak,
    Immune,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct DamageModifier {
    modifier_type: ModifierType,
    damage_type: DamageType,
}

impl DamageModifier {
    fn weak(damage_type: DamageType) -> DamageModifier {
        DamageModifier {
            modifier_type: ModifierType::Weak,
            damage_type,
        }
    }
    fn immune(damage_type: DamageType) -> DamageModifier {
        DamageModifier {
            modifier_type: ModifierType::Immune,
            damage_type,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Attack {
    amount: u32,
    attack_type: DamageType,
}

impl Attack {
    fn boost(&self, amount: u32) -> Self {
        Self {
            amount: self.amount + amount,
            ..*self
        }
    }

    fn get_multiplier(&self, modifiers: &[DamageModifier]) -> u32 {
        let modifier_type = modifiers
            .iter()
            .find_map(|x| (x.damage_type == self.attack_type).then_some(x.modifier_type));

        match modifier_type {
            Some(ModifierType::Weak) => 2,
            None => 1,
            Some(ModifierType::Immune) => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Side {
    Infection,
    ImmuneSystem,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Group {
    side: Side,
    units: u32,
    hit_points: u32,
    modifiers: Vec<DamageModifier>,
    attack: Attack,
    initiative: u32,
}

impl Group {
    fn boost(&self, amount: u32) -> Self {
        Self {
            attack: self.attack.boost(amount),
            ..self.clone()
        }
    }

    fn is_active(&self) -> bool {
        self.units > 0
    }

    fn effective_power(&self) -> u32 {
        self.units * self.attack.amount
    }

    fn damage_dealt(&self, other: &Group) -> u32 {
        self.effective_power() * self.attack.get_multiplier(&other.modifiers)
    }

    fn take_damage(&mut self, amount: u32) -> u32 {
        if amount == 0 {
            return 0;
        }
        let dead = (amount / self.hit_points).min(self.units);
        self.units = self.units.saturating_sub(dead);

        dead
    }
}

fn sort_groups(a: &Group, b: &Group) -> std::cmp::Ordering {
    b.effective_power()
        .cmp(&a.effective_power())
        .then(b.initiative.cmp(&a.initiative))
}

type Targeting<'a, 'b> = &'a (usize, &'b Group, u32);
fn targeting_sort(
    (_a_idx, a, a_dmg): Targeting,
    (_b_idx, b, b_dmg): Targeting,
) -> std::cmp::Ordering {
    b_dmg
        .cmp(a_dmg)
        .then(b.effective_power().cmp(&a.effective_power()))
        .then(b.initiative.cmp(&a.initiative))
}

fn get_targets(groups: &[Group]) -> Vec<(usize, usize)> {
    let mut already_targeted: HashSet<usize> = HashSet::new();

    groups
        .iter()
        .enumerate()
        .filter_map(|(g_idx, g)| {
            let target = groups
                .iter()
                .enumerate()
                .filter_map(|(x_idx, x)| {
                    let is_enemy = g.side != x.side;
                    let not_targeted = !already_targeted.contains(&x_idx);
                    let damage = g.damage_dealt(x);
                    let valid_target = is_enemy && not_targeted && damage != 0;

                    valid_target.then_some((x_idx, x, damage))
                })
                .sorted_by(targeting_sort)
                .map(|(idx, _group, _damage)| idx)
                .next();

            target.map(|target| {
                already_targeted.insert(target);
                (g_idx, target)
            })
        })
        .collect_vec()
}

#[derive(Debug)]
struct BattleResult {
    immune: u32,
    infection: u32,
}

impl BattleResult {
    fn immune_win(&self) -> bool {
        self.immune > 0 && self.infection == 0
    }
}

fn battle(input: &Input) -> BattleResult {
    let mut units = input.clone();

    while units.iter().any(|x| x.side == Side::ImmuneSystem)
        && units.iter().any(|x| x.side == Side::Infection)
    {
        // sort by effective power, then initiative
        units.sort_by(sort_groups);

        // get all the targets and sort by attacker initiative
        let pairs = get_targets(&units)
            .into_iter()
            .sorted_by_key(|x| Reverse(units[x.0].initiative));

        let mut units_killed = 0;
        // execute the attacks
        for (attacker, defender) in pairs {
            let attacker = units.get(attacker).cloned().unwrap();

            if let Some(defender) = units.get_mut(defender) {
                let damage = attacker.damage_dealt(defender);
                units_killed += defender.take_damage(damage);
            }
        }

        // some boosts in part 2 wind up with no units dying in a battle, those are hopeless
        if units_killed == 0 {
            break;
        }

        // drop any that are all dead
        units.retain(|x| x.is_active());
    }

    // how many units remain?
    BattleResult {
        immune: units
            .iter()
            .filter_map(|x| (x.side == Side::ImmuneSystem).then_some(x.units))
            .sum(),
        infection: units
            .iter()
            .filter_map(|x| (x.side == Side::Infection).then_some(x.units))
            .sum(),
    }
}

fn boosted_battle(input: &Input, amount: u32) -> BattleResult {
    let boosted = input
        .iter()
        .map(|x| match x.side {
            Side::Infection => x.clone(),
            Side::ImmuneSystem => x.boost(amount),
        })
        .collect();

    battle(&boosted)
}

fn problem1(input: &Input) -> u32 {
    battle(input).infection
}

fn problem2(input: &Input) -> u32 {
    for n in 0.. {
        let result = boosted_battle(input, n);
        if result.immune_win() {
            return result.immune;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{boosted_battle, parse, problem1, Attack, DamageType, Group, Side};
    #[test]
    fn damage() {
        let mut group = Group {
            units: 10,
            hit_points: 10,
            modifiers: vec![],
            side: Side::ImmuneSystem,
            attack: Attack {
                amount: 0,
                attack_type: DamageType::Fire,
            },
            initiative: 10,
        };

        group.take_damage(75);
        assert_eq!(group.units, 3);

        group.take_damage(75);
        assert_eq!(group.units, 0);
        assert!(!group.is_active());
    }
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 5216)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = boosted_battle(&input, 1570);
        assert_eq!(result.immune, 51);
    }
}
