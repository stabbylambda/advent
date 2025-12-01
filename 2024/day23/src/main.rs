use itertools::Itertools;
use nom::{
    bytes::complete::take,
    character::complete::{char, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input<'a> = Vec<(&'a str, &'a str)>;

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(take(2usize), char('-'), take(2usize)),
    ).parse(input);

    result.unwrap().1
}

fn get_computers<'a>(input: &'a Input) -> HashMap<&'a str, Vec<&'a str>> {
    input.iter().fold(HashMap::new(), |mut acc, (a, b)| {
        acc.entry(*a)
            .and_modify(|x: &mut Vec<&str>| x.push(b))
            .or_insert(vec![b]);
        acc.entry(*b)
            .and_modify(|x: &mut Vec<&str>| x.push(a))
            .or_insert(vec![a]);
        acc
    })
}

fn problem1(input: &Input) -> usize {
    let computers = get_computers(input);
    let mut triples: HashSet<Vec<&str>> = HashSet::new();
    for (a, x) in computers.iter().filter(|(k, _)| k.starts_with("t")) {
        for &b in x {
            for c in computers.get(&b).unwrap() {
                if computers.get(c).unwrap().contains(a) {
                    let mut triple = vec![a, b, c];
                    triple.sort();
                    triples.insert(triple);
                }
            }
        }
    }

    triples.len()
}

// https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm
fn find_cliques<'a>(
    computers: &HashMap<&'a str, Vec<&'a str>>,
    potential: Vec<&'a str>,
    mut remaining: Vec<&'a str>,
    mut skip: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    if remaining.is_empty() && skip.is_empty() {
        return vec![potential];
    }

    let mut cliques = vec![];

    while let Some(node) = remaining.pop() {
        let mut new_potential = potential.clone();
        new_potential.push(node);

        let Some(neighbors) = computers.get(node) else {
            continue;
        };

        let new_remaining = remaining
            .iter()
            .filter(|x| neighbors.contains(x))
            .cloned()
            .collect_vec();
        let new_skip = skip
            .iter()
            .filter(|x| neighbors.contains(x))
            .cloned()
            .collect_vec();

        let result = find_cliques(computers, new_potential, new_remaining, new_skip);
        cliques.extend_from_slice(&result);

        skip.push(node);
    }

    cliques
}

fn problem2(input: &Input) -> String {
    let computers = get_computers(input);

    let nodes = computers.keys().cloned().collect_vec();
    let result = find_cliques(&computers, vec![], nodes.clone(), vec![]);
    let mut max_clique = result
        .iter()
        .sorted_by_key(|x| Reverse(x.len()))
        .next()
        .cloned()
        .unwrap();

    max_clique.sort();
    max_clique.join(",")
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 7)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, "co,de,ka,ta".to_string())
    }
}
