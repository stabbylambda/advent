use std::collections::HashSet;

fn main() {
    let lines = include_str!("../input.txt");
    let rucksacks = get_rucksacks(lines);

    let score = problem1(&rucksacks);
    println!("problem 1 score: {score}");

    let score = problem2(&rucksacks);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl Rucksack {
    fn new(s: &str) -> Rucksack {
        let (c1, c2) = s.split_at(s.len() / 2);
        Rucksack {
            compartment1: HashSet::from_iter(c1.chars()),
            compartment2: HashSet::from_iter(c2.chars()),
        }
    }

    fn get_shared(&self) -> &char {
        self.compartment1
            .intersection(&self.compartment2)
            .next()
            .unwrap()
    }

    fn get_all_items(&self) -> HashSet<&char> {
        self.compartment1.union(&self.compartment2).collect()
    }

    fn get_badge<'a>(&'a self, other1: &'a Rucksack, other2: &'a Rucksack) -> &char {
        let s0: HashSet<_> = self.get_all_items();
        let s1: HashSet<_> = other1.get_all_items();
        let s2: HashSet<_> = other2.get_all_items();

        let common1: HashSet<_> = s0.intersection(&s1).collect();
        let common2: HashSet<_> = s1.intersection(&s2).collect();

        common1.intersection(&common2).next().unwrap()
    }
}

fn prioritize(c: &char) -> u32 {
    (*c as u32) - if c.is_lowercase() { 96 } else { 38 }
}

fn get_rucksacks(lines: &str) -> Vec<Rucksack> {
    lines.lines().map(Rucksack::new).collect::<Vec<Rucksack>>()
}

fn problem1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().map(|r| prioritize(r.get_shared())).sum()
}

fn problem2(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .chunks(3)
        .map(|g| {
            let [g0, g1, g2] = g else {
                panic!("not a valid group");
            };
            prioritize(g0.get_badge(g1, g2))
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{get_rucksacks, problem1, problem2};
    #[test]
    fn first() {
        let lines = include_str!("../test.txt");
        let rucksacks = get_rucksacks(lines);
        let result = problem1(&rucksacks);
        assert_eq!(result, 157);
    }

    #[test]
    fn second() {
        let lines = include_str!("../test.txt");
        let rucksacks = get_rucksacks(lines);
        let result = problem2(&rucksacks);
        assert_eq!(result, 70)
    }
}
