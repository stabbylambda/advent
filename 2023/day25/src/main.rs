use std::collections::{BTreeMap, BTreeSet};

use common::dijkstra::{connected_components, Edge};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    println!("Merry Christmas!");
}

type Input<'a> = Vec<(&'a str, Vec<&'a str>)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    // unique the connections
    let all_wires: BTreeSet<(&str, &str)> = input
        .iter()
        .flat_map(|(from, tos)| tos.iter().map(|to| (*from, *to)))
        .collect();

    // unique the components
    let all_components: BTreeMap<&str, usize> = all_wires
        .iter()
        .flat_map(|(from, to)| vec![from, to])
        .unique()
        .enumerate()
        .map(|(i, s)| (*s, i))
        .collect();

    // get the adjacency matrix
    let canonical_edges = vec![vec![]; all_components.len()];
    for (from, tos) in input {
        for to in tos {
            let from_idx = all_components[*from];
            let to_idx = all_components[*to];

            // println!("{from} ({from_idx}) -> {to} ({to_idx})");
            canonical_edges[from_idx].push(Edge {
                node: to_idx,
                cost: 1,
            });
            canonical_edges[to_idx].push(Edge {
                node: from_idx,
                cost: 1,
            });
        }
    }

    all_wires
        .iter()
        .permutations(3)
        .par_bridge()
        .find_map_first(|skip| {
            let c = connected_components(&edges);
            (c.len() == 2).then(|| c.iter().map(|(k, v)| v.len()).product())
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 54)
    }
}
