use std::collections::{BTreeSet, HashMap};

use petgraph::{graph::NodeIndex, prelude::UnGraph, Graph};
fn main() {
    let input = include_str!("../input.txt");
    let graph = GraphData::parse(input);

    let answer = problem1(&graph);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&graph);
    println!("problem 2 answer: {answer}");
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default, Clone, Copy)]
struct Cave<'a> {
    value: &'a str,
    is_small: bool,
    is_start: bool,
    is_end: bool,
}
impl<'a> Cave<'a> {
    fn new(s: &'a str) -> Cave<'a> {
        Cave {
            value: s,
            is_small: s.chars().all(|x| x.is_lowercase()),
            is_start: s == "start",
            is_end: s == "end",
        }
    }
}

struct GraphData<'a> {
    graph: UnGraph<Cave<'a>, ()>,
    start: NodeIndex,
}
impl<'a> GraphData<'a> {
    fn parse(input: &str) -> GraphData<'_> {
        let mut graph: UnGraph<Cave, ()> = Graph::new_undirected();
        let edges: Vec<(Cave, Cave)> = input
            .lines()
            .map(|line| {
                let x: Vec<Cave> = line.split('-').map(Cave::new).collect();
                (x[0], x[1])
            })
            .collect();

        let caves: BTreeSet<Cave> = edges.iter().flat_map(|(c1, c2)| vec![*c1, *c2]).collect();
        let cave_indices: HashMap<Cave, _> =
            caves.iter().map(|x| (*x, graph.add_node(*x))).collect();

        graph.extend_with_edges(
            edges
                .iter()
                .map(|(c1, c2)| (cave_indices[c1], cave_indices[c2])),
        );
        let start = *cave_indices
            .iter()
            .find(|(cave, _)| cave.is_start)
            .unwrap()
            .1;

        GraphData { graph, start }
    }

    fn paths_to_end(&self, allow_double_visit: bool) -> usize {
        self._paths_to_end(self.start, HashMap::new(), allow_double_visit)
    }

    fn _paths_to_end(
        &self,
        start: NodeIndex,
        visited: HashMap<NodeIndex, u32>,
        allow_double_visit: bool,
    ) -> usize {
        // visit each neighbor of the current node
        self.graph.neighbors(start).fold(0, |acc, n| {
            let neighbor = self.graph[n];
            let times_visited = *visited.get(&n).unwrap_or(&0);

            acc + if neighbor.is_end {
                // if we arrived at the end, we're done
                1
            } else if neighbor.is_start
                || (neighbor.is_small && !allow_double_visit && times_visited > 0)
            {
                // never visit start again
                // already double visited, don't go down this path anymore
                0
            } else {
                let allow_double_visit =
                    allow_double_visit && (!neighbor.is_small || times_visited < 1);

                let mut visited = visited.clone();
                *(visited.entry(n).or_insert(0)) += 1;

                self._paths_to_end(n, visited, allow_double_visit)
            }
        })
    }
}

fn problem1(graph_data: &GraphData) -> usize {
    graph_data.paths_to_end(false)
}

fn problem2(graph_data: &GraphData) -> usize {
    graph_data.paths_to_end(true)
}

#[cfg(test)]
mod test {

    use crate::{problem1, problem2, GraphData};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let graph = GraphData::parse(input);
        let result = problem1(&graph);
        assert_eq!(result, 10)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let graph = GraphData::parse(input);
        let result = problem2(&graph);
        assert_eq!(result, 36)
    }
}
