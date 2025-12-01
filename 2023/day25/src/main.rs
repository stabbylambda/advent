use std::collections::BTreeSet;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graphmap::UnGraphMap, Result};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    println!("Merry Christmas!");
}

type Input<'a> = Vec<(&'a str, Vec<&'a str>)>;

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    // flatten the connections
    let all_wires: BTreeSet<(&str, &str)> = input
        .iter()
        .flat_map(|(from, tos)| tos.iter().map(|to| (*from, *to)))
        .collect();

    let g = UnGraphMap::<&str, ()>::from_edges(&all_wires);

    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&g, |_| Ok(1));
    let (wires_to_cut, p1) = min_cut_res.unwrap().unwrap();

    if wires_to_cut != 3 {
        panic!("Something went wrong")
    }

    let size1 = p1.len();
    let size2 = g.node_count() - p1.len();

    size1 * size2
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
