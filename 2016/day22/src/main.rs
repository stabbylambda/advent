use itertools::Itertools;
use std::fmt::Display;

use common::{
    dijkstra::{shortest_path, Edge},
    grid::Grid,
};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Node>;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    avail: u32,
    used_percent: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Node {
    fn is_empty(&self) -> bool {
        self.used == 0
    }
}

fn parse(input: &str) -> Input {
    let name = preceded(
        tag("/dev/grid/node-"),
        separated_pair(preceded(tag("x"), u32), tag("-"), preceded(tag("y"), u32)),
    );

    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            (
                terminated(name, multispace1),
                terminated(terminated(u32, tag("T")), multispace1),
                terminated(terminated(u32, tag("T")), multispace1),
                terminated(terminated(u32, tag("T")), multispace1),
                terminated(u32, tag("%")),
            ),
            |((x, y), size, used, avail, used_percent)| Node {
                x,
                y,
                size,
                used,
                avail,
                used_percent,
            },
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input
        .iter()
        .filter(|a| !a.is_empty())
        .flat_map(|a| input.iter().filter_map(move |b| (a != b).then_some((a, b))))
        // .inspect(|(x, y)| println!("{x} - {y}"))
        .filter(|(a, b)| a.used <= b.avail)
        .count()
}

fn get_edges(maze: &Grid<&Node>) -> Vec<Vec<Edge>> {
    maze.iter()
        .map(|square| {
            // large nodes are walls and have no edges
            if square.data.size > 100 {
                return vec![];
            }

            square
                .neighbors()
                .iter()
                .filter_map(|n| {
                    if n.data.size > 100 {
                        return None;
                    }

                    Some(Edge::from_map_square(n))
                })
                .collect()
        })
        .collect()
}

fn problem2(input: &Input) -> usize {
    let empty = input.iter().find(|n| n.used == 0).unwrap();
    let goal = input.iter().filter(|n| n.y == 0).max().unwrap();

    let grid = input
        .iter()
        .sorted()
        .chunk_by(|n| n.y)
        .into_iter()
        .map(|(_, v)| v.collect_vec())
        .collect_vec();

    let map = common::grid::Grid::new(grid);

    println!("========= Grid Visualization =========");
    map.print(|s| {
        let n = *s.data;
        if n == goal {
            'G'
        } else if n == empty {
            '_'
        } else if n.size > 100 {
            '#'
        } else {
            '.'
        }
    });
    println!("======================================");

    let empty_index = map
        .get((empty.x as usize, empty.y as usize))
        .get_grid_index();

    let next_to_goal_index = map
        .get((goal.x as usize - 1, goal.y as usize))
        .get_grid_index();

    let edges = get_edges(&map);
    // move the empty up next to the left of the goal, with the large nodes as 'walls'
    let move_empty_next_to_goal = shortest_path(&edges, empty_index, next_to_goal_index).unwrap();

    /* once we do that, it's a matter of doing the sliding puzzle like so:
    ._G   .G_   .G.   .G.   .G.   _G.
    ...   ...   .._   ._.   _..   ...

    for each cell between the goal and the origin, which is 5 moves times the number of cells between the goal
    and the origin
    */
    let move_goal_to_origin = 5 * (goal.x as usize - 1);

    // plus one for the extra move
    move_empty_next_to_goal + move_goal_to_origin + 1
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 903)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 215)
    }
}
