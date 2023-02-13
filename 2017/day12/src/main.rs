use std::collections::{BinaryHeap, HashMap};

use common::dijkstra::Edge;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input = Vec<Vec<Edge>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        preceded(
            terminated(u32, tag(" <-> ")),
            separated_list1(tag(", "), map(u32, |x| Edge::new(x as usize))),
        ),
    )(input);

    result.unwrap().1
}

fn problem(input: &Input) -> (usize, usize) {
    let connected = connected_components(input);
    let zero_group_len = connected[&0].len();
    let total = connected.len();

    (zero_group_len, total)
}

fn connected_components(input: &Vec<Vec<Edge>>) -> HashMap<usize, Vec<usize>> {
    let mut visited = vec![false; input.len()];
    let mut groups = HashMap::new();

    for v in 0..input.len() {
        if !visited[v] {
            let mut group = vec![];
            let mut queue = BinaryHeap::new();
            queue.push(v);
            visited[v] = true;

            // bfs through the adjacency list and find all the components connected to v
            while let Some(v1) = queue.pop() {
                group.push(v1);
                for e in &input[v1] {
                    let v2 = e.node;
                    if !visited[v2] {
                        visited[v2] = true;
                        queue.push(v2);
                    }
                }
            }

            groups.insert(v, group);
        }
    }

    groups
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (zero_len, total) = problem(&input);
        assert_eq!(zero_len, 6);
        assert_eq!(total, 2);
    }
}
