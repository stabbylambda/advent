use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<(char, char)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        separated_pair(
            preceded(tag("Step "), anychar),
            tag(" must be finished before step "),
            terminated(anychar, tag(" can begin.")),
        ),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> String {
    // construct the graph of incoming edges
    let mut incoming: HashMap<char, HashSet<char>> = HashMap::new();
    for &(from, to) in input {
        incoming.entry(from).or_default();
        incoming
            .entry(to)
            .and_modify(|v| {
                v.insert(from);
            })
            .or_insert_with(|| HashSet::from_iter([from]));
    }

    // The example has a single root, but the actual input has multiple.
    let mut s: BinaryHeap<Reverse<char>> = incoming
        .iter()
        .filter_map(|(k, v)| v.is_empty().then_some(Reverse(*k)))
        .collect();

    let mut steps = vec![];

    // This is https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    // using a BinaryHeap with Reverse gives us the lexicographical sorting that the problem requires
    while let Some(Reverse(next)) = s.pop() {
        // add n to the final result
        steps.push(next.to_string());

        // Find all edges from n -> m (it's easier to do this from the original list)
        for (n, m) in input.iter().filter(|(n, _m)| (*n == next)) {
            // remove the edge
            if let Some(v) = incoming.get_mut(m) {
                v.remove(n);
            };

            // if m has no other incoming edges, add m to the heap
            if incoming[m].is_empty() {
                s.push(Reverse(*m));
            }
        }
    }

    steps.join("")
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
        assert_eq!(result, "CABDFE")
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
