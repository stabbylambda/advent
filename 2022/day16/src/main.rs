use ndarray::prelude::*;
use std::collections::BTreeSet;

pub mod parser;

fn main() {
    let input = include_str!("../input.txt");
    let input = parser::parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Caves;
type ValveId = usize;

#[derive(Debug)]
struct Valve {
    id: ValveId,
    flow_rate: u32,
    neighbors: Vec<usize>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct Caves {
    valves: Vec<Valve>,
    aa_index: ValveId,
    distances: Array2<u32>,
}
impl Caves {
    fn new(valves: Vec<Valve>, aa_index: ValveId) -> Caves {
        let distances = floyd_warshall(&valves);

        Caves {
            valves,
            aa_index,
            distances,
        }
    }

    fn non_zero_valves(&self) -> Vec<&Valve> {
        self.valves.iter().filter(|x| x.flow_rate != 0).collect()
    }
}

fn floyd_warshall(valves: &[Valve]) -> Array2<u32> {
    // initialize with an arbitrarily large number that isn't u32::MAX because I don't want to overflow
    let mut dist = Array2::from_elem((valves.len(), valves.len()), 100_000);
    for (v, valve) in valves.iter().enumerate() {
        for &e in &valve.neighbors {
            dist[[v, e]] = 1;
        }
        dist[[v, v]] = 0;
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[[i, j]] = dist[[i, j]].min(dist[[i, k]] + dist[[k, j]])
            }
        }
    }

    dist
}

type Path<'a> = Vec<&'a Valve>;

/* A recursive DFS to go through all the remaining valves that we have time to open. We skip a ton of the problem
space by using the distances that we precomputed with the floyd warshall algorithm because we're able to omit visiting
every node that has a valve that's zero. What we'll end up with is a Vec of Paths that we can score after.

It would likely be faster to keep track of the scores on the way back up and omit whole sections that are worse...but I
think this runs Fast Enough (tm). */
fn find_all_paths<'a>(
    caves: &'a Caves,
    position: usize,
    opened_valves: Vec<&'a Valve>,
    time_left: u32,
) -> Vec<Path<'a>> {
    // get a list of the non-opened valves that we have time to get to
    let remaining: Vec<(&Valve, u32)> = caves
        .non_zero_valves()
        .iter()
        .filter_map(|next| {
            let dist = caves.distances[[position, next.id]];
            if !opened_valves.contains(next) && dist < time_left {
                Some((*next, dist))
            } else {
                None
            }
        })
        .collect();

    let current_path = opened_valves.clone();
    // we want to go over all the remaining paths
    let mut remaining_paths: Vec<Path> = remaining
        .iter()
        .flat_map(|(x, dist)| {
            // get the remaining time to move there and open a valve
            let new_time_left = time_left - dist - 1;

            // push the new valve onto the stack
            let mut new_opened_valves = opened_valves.clone();
            new_opened_valves.push(*x);

            // and recursively find all the remaining paths
            find_all_paths(caves, x.id, new_opened_valves, new_time_left)
        })
        .collect();

    remaining_paths.push(current_path);
    remaining_paths
}

fn path_score(caves: &Caves, path: &Path, time_left: u32) -> u32 {
    let (_, _, score) = path.iter().fold(
        (caves.aa_index, time_left, 0),
        |(pos, time_left, score), v| {
            // get the remaining time to get to this node and then open the valve
            let time_left = time_left - caves.distances[[pos, v.id]] - 1;
            // calculate the new score based off the steam we'll release from this valve
            let score = score + v.flow_rate * time_left;

            (v.id, time_left, score)
        },
    );

    score
}

fn problem1(caves: &Input) -> u32 {
    let time_left = 30;

    // find all the possible paths through the maze
    let all_paths = find_all_paths(caves, caves.aa_index, Vec::new(), time_left);
    // score all the paths
    let path_scores = all_paths
        .iter()
        .map(|path| path_score(caves, path, time_left));

    // get the max
    path_scores.max().unwrap()
}

fn problem2(caves: &Input) -> u32 {
    let time_left = 26;

    // find all the possible paths through the maze
    let all_paths = find_all_paths(caves, caves.aa_index, Vec::new(), time_left);
    // score all the paths
    let mut path_scores: Vec<(BTreeSet<ValveId>, u32)> = all_paths
        .iter()
        .map(|path| {
            (
                // we're going to need to do set comparison later, so just make all the paths sets
                // note: we probably could have done this and the pathing with a bitvec? would that have been faster?
                path.iter().map(|v| v.id).collect(),
                path_score(caves, path, time_left),
            )
        })
        .collect();

    // order them by their scores so we can start by comparing best scores
    path_scores.sort_by(|(_, sa), (_, sb)| sb.cmp(sa));

    let mut answer = 0;

    // there are now two actors exploring possible paths, so we need to model that
    for human_idx in 0..path_scores.len() {
        let (human_path, human_score) = &path_scores[human_idx];

        // quit as soon as we found there are no more possible best answers
        if human_score + path_scores[human_idx + 1].1 < answer {
            break;
        }

        // only compare scores that are smaller than our current score
        (human_idx + 1..path_scores.len()).for_each(|elephant_idx| {
            let (elephant_path, elephant_score) = &path_scores[elephant_idx];

            // we want the best score where both the human and elephant open disjoint sets of valves
            if human_path.is_disjoint(elephant_path) {
                answer = answer.max(human_score + elephant_score);
            }
        });
    }

    answer
}

#[cfg(test)]
mod test {

    use crate::{parser::parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 1651)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 1707)
    }
}
