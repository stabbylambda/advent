use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input, 5, 60);
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
    ).parse(input);

    result.unwrap().1
}

struct Graph {
    incoming: HashMap<char, HashSet<char>>,
    roots: BinaryHeap<Reverse<char>>,
    nodes: Vec<char>,
}

impl Graph {
    fn new(input: &Input) -> Graph {
        let incoming = Graph::get_incoming(input);

        let from_set: HashSet<char> = input.iter().map(|(from, _to)| *from).collect();
        let to_set: HashSet<char> = input.iter().map(|(_from, to)| *to).collect();
        let roots = Graph::get_roots(&from_set, &to_set);
        let nodes = from_set.union(&to_set).cloned().collect();

        Graph {
            incoming,
            roots,
            nodes,
        }
    }
    fn get_incoming(input: &Input) -> HashMap<char, HashSet<char>> {
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

        incoming
    }

    fn get_roots(from_set: &HashSet<char>, to_set: &HashSet<char>) -> BinaryHeap<Reverse<char>> {
        from_set.difference(to_set).map(|x| Reverse(*x)).collect()
    }
}

fn problem1(input: &Input) -> String {
    let mut graph = Graph::new(input);
    let mut available = graph.roots.clone();
    let mut steps = vec![];

    // This is https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    // using a BinaryHeap with Reverse gives us the lexicographical sorting that the problem requires
    while let Some(Reverse(next)) = available.pop() {
        // add n to the final result
        steps.push(next.to_string());

        // Find all edges from n -> m (it's easier to do this from the original list)
        for (n, m) in input.iter().filter(|(n, _m)| *n == next) {
            // remove the edge
            if let Some(v) = graph.incoming.get_mut(m) {
                v.remove(n);
            };

            // if m has no other incoming edges, add m to the heap
            if graph.incoming[m].is_empty() {
                available.push(Reverse(*m));
            }
        }
    }

    steps.join("")
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Job {
    letter: char,
    time_left: u32,
}

impl Job {
    fn new(job: char, base: u32) -> Job {
        Job {
            letter: job,
            time_left: (job as u32 - 'A' as u32) + 1 + base,
        }
    }
}

// This is suuuuuper ugly. There has to be a better way to do this, but I don't think I care right now
fn problem2(input: &Input, workers: usize, base: u32) -> u32 {
    let mut graph = Graph::new(input);
    let mut available = graph.roots.clone();

    // create the job pool
    let mut all_jobs: HashMap<char, Job> = graph
        .nodes
        .iter()
        .map(|c| (*c, Job::new(*c, base)))
        .collect();

    let mut workers: Vec<Option<Job>> = vec![None; workers];
    let mut t = 0;

    while !all_jobs.is_empty() || workers.iter().any(|x| x.is_some()) {
        for assigned in workers.iter_mut() {
            // check if we can assign a job to this worker
            if assigned.is_none() {
                // check if there are any jobs to assign
                if let Some(Reverse(job)) = available.pop() {
                    *assigned = all_jobs.remove(&job);
                }
            }

            // if this worker has been assigned a job
            if let Some(job) = assigned {
                // decrement the count
                job.time_left -= 1;

                if job.time_left == 0 {
                    // clear this requirement from any of the nodes that depend on it
                    for (k, v) in graph.incoming.iter_mut() {
                        if v.remove(&job.letter) && v.is_empty() {
                            // if the job is empty, push it onto the available heap
                            available.push(Reverse(*k));
                        }
                    }

                    *assigned = None;
                }
            }
        }

        t += 1;
    }

    t
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
        let result = problem2(&input, 2, 0);
        assert_eq!(result, 15)
    }
}
