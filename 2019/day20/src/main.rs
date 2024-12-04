use std::collections::{BTreeMap, BinaryHeap, HashMap};

use common::{
    dijkstra::{shortest_path, Edge},
    grid::{Grid, GridSquare},
    nom::parse_grid,
};
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::map,
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

type Input = PlutoMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
    Void,
    Label(char),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        parse_grid(alt((
            map(char('#'), |_| Tile::Wall),
            map(char('.'), |_| Tile::Space),
            map(char(' '), |_| Tile::Void),
            map(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), Tile::Label),
        ))),
        PlutoMap::new,
    )(input);

    result.unwrap().1
}
struct PlutoMap {
    edges: Vec<Vec<Edge>>,
    portals: PortalMap,
}

impl PlutoMap {
    fn new(input: Grid<Tile>) -> Self {
        let edges = PlutoMap::get_edges(&input);
        let portals = PlutoMap::get_portals(&input);

        PlutoMap { edges, portals }
    }

    fn get_portals(map: &Grid<Tile>) -> PortalMap {
        let mut portals: HashMap<PortalKey, Vec<PortalEndpoint>> = HashMap::new();
        for y in 0..map.height {
            for x in 0..map.width {
                let tile = map.get((x, y));
                if let Some((a, b, grid_index)) = Self::get_portal_info(&tile) {
                    // make the key by sorting, since a and b won't always be in the same order
                    let key = (a.min(b), a.max(b));
                    let ends = portals.entry(key).or_default();

                    // // figure out if the portal is on the inner ring or the outer ring
                    let outer = y == 1 || x == 1 || y == map.height - 2 || x == map.width - 2;
                    ends.push(PortalEndpoint { grid_index, outer });
                }
            }
        }

        PortalMap::new(portals)
    }

    fn get_edges(maze: &Grid<Tile>) -> Vec<Vec<Edge>> {
        // Just construct a normal adjacency list, where walls, voids, and labels have no edges
        // we'll add in the portal edges later
        maze.into_iter()
            .map(|square| {
                if square.data != &Tile::Space {
                    return vec![];
                }

                square
                    .neighbors()
                    .to_vec()
                    .iter()
                    .filter_map(|n| match n.data {
                        Tile::Space => Some(Edge::from_map_square(n)),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }

    fn get_portal_info(tile: &GridSquare<Tile>) -> Option<(char, char, usize)> {
        // if this isn't a label, just quit
        let &Tile::Label(a) = tile.data else {
            return None;
        };

        let neighbors = tile.neighbors();
        // now find the neighboring label and space
        let label_neighbor = neighbors.into_iter().find_map(|x| match x.data {
            Tile::Label(b) => Some(*b),
            _ => None,
        });
        let space_neighbor = neighbors.into_iter().find_map(|x| match x.data {
            Tile::Space => Some(x.get_grid_index()),
            _ => None,
        });

        // if we have both, we're good
        if let Some((b, grid_index)) = label_neighbor.zip(space_neighbor) {
            Some((a, b, grid_index))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PortalEndpoint {
    grid_index: usize,
    outer: bool,
}

type PortalKey = (char, char);

#[derive(Clone, Copy, Debug)]
struct Portal {
    key: PortalKey,
    entrance: usize,
    exit: usize,
    level_diff: i32,
}

impl Portal {
    fn new(key: PortalKey, entrance: usize, exit: usize, up: bool) -> Self {
        Portal {
            key,
            entrance,
            exit,
            level_diff: match up {
                true => -1,
                false => 1,
            },
        }
    }

    fn is_down(&self) -> bool {
        self.level_diff == 1
    }
}

const AA: (char, char) = ('A', 'A');
const ZZ: (char, char) = ('Z', 'Z');

struct PortalMap {
    map: HashMap<PortalKey, Vec<PortalEndpoint>>,
    endpoints: Vec<Portal>,
}

impl PortalMap {
    fn new(map: HashMap<PortalKey, Vec<PortalEndpoint>>) -> Self {
        let endpoints = Self::endpoints(&map);
        Self { map, endpoints }
    }

    fn aa_index(&self) -> usize {
        self.map[&AA].first().unwrap().grid_index
    }

    fn zz_index(&self) -> usize {
        self.map[&ZZ].first().unwrap().grid_index
    }

    fn endpoints(map: &HashMap<PortalKey, Vec<PortalEndpoint>>) -> Vec<Portal> {
        map.iter()
            .flat_map(|(k, v)| {
                let inner = v.iter().find_map(|x| (!x.outer).then_some(x.grid_index));
                let outer = v.iter().find_map(|x| (x.outer).then_some(x.grid_index));

                match (inner, outer) {
                    (Some(inner), Some(outer)) => {
                        let down = Portal::new(*k, inner, outer, false);
                        let up = Portal::new(*k, outer, inner, true);

                        vec![down, up]
                    }
                    // this is just AA and ZZ
                    (None, Some(outer)) if k == &ZZ => {
                        vec![Portal::new(*k, outer, outer, true)]
                    }
                    _ => vec![],
                }
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    level: i32,
    steps: usize,
    portal_key: PortalKey,
    portal_idx: usize,
}

impl State {
    fn validate_portal(&self, portal: &Portal) -> bool {
        let is_aa_zz = portal.key == AA || portal.key == ZZ;

        let valid_for_level = if self.level == 0 {
            // on level zero, only the up portals aa and zz work
            is_aa_zz
        } else {
            // on lower levels, aa and zz are walls
            !is_aa_zz
        };

        // down portals always work
        portal.key != self.portal_key && (portal.is_down() || valid_for_level)
    }

    fn go_through(&self, portal: &Portal, steps: usize) -> Self {
        let mut state = *self;

        // move through the portal
        state.steps += steps;
        state.level += portal.level_diff;
        state.portal_key = portal.key;
        state.portal_idx = portal.exit;
        state
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reversed so we can do a min heap
        other.steps.cmp(&self.steps)
    }
}

fn problem1(input: &Input) -> usize {
    let mut edges = input.edges.to_vec();

    // add the paired endpoints to the adjacency list
    for portal in &input.portals.endpoints {
        edges[portal.entrance].push(Edge::new(portal.exit));
    }

    shortest_path(&edges, input.portals.aa_index(), input.portals.zz_index()).unwrap()
}

fn problem2(input: &Input) -> usize {
    let state = State {
        level: 0,
        steps: 0,
        portal_key: AA,
        portal_idx: input.portals.aa_index(),
    };

    // store the paths in a cache because the number of steps never change
    let mut steps_cache: BTreeMap<(usize, usize), Option<usize>> = BTreeMap::new();

    let mut queue = BinaryHeap::new();
    queue.push(state);

    while let Some(state) = queue.pop() {
        // did we get to the end?
        if state.portal_key == ZZ {
            return state.steps - 1;
        }

        // go through all the portals that are possible from here
        let new_states = input
            .portals
            .endpoints
            .iter()
            .filter(|p| state.validate_portal(p))
            .filter_map(|portal| {
                // get the steps from the cache or calculate it
                let steps = steps_cache
                    .entry((state.portal_idx, portal.entrance))
                    .or_insert_with(|| {
                        shortest_path(&input.edges, state.portal_idx, portal.entrance)
                    });

                steps.map(|steps| state.go_through(portal, steps + 1))
            });

        queue.extend(new_states);
    }

    0
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 58)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 396)
    }
}
