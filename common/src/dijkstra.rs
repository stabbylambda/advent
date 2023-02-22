use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::map::MapSquare;

#[derive(Debug)]
pub struct Edge {
    pub node: usize,
    pub cost: usize,
}

impl Edge {
    pub fn new(node: usize) -> Edge {
        Edge { node, cost: 1 }
    }

    pub fn from_map_square<T: Copy>(square: &MapSquare<T>) -> Edge {
        Edge {
            node: square.get_grid_index(),
            cost: 1,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type GridIndex = usize;

pub fn shortest_path(
    adj_list: &Vec<Vec<Edge>>,
    start: GridIndex,
    goal: GridIndex,
) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

pub fn connected_components(input: &Vec<Vec<Edge>>) -> HashMap<usize, Vec<usize>> {
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
