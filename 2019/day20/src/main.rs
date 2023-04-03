use std::collections::HashMap;

use common::{
    dijkstra::{shortest_path, Edge},
    map::{Map, MapSquare},
};
use nom::{
    branch::alt,
    character::complete::{char, newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
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

type Input = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
    Void,
    Label(char),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(char('#'), |_| Tile::Wall),
            map(char('.'), |_| Tile::Space),
            map(char(' '), |_| Tile::Void),
            map(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), Tile::Label),
        ))),
    )(input);

    result.unwrap().1
}

fn label_neighbor(tile: &MapSquare<Tile>) -> Option<char> {
    let neighbors = tile.neighbors();
    neighbors.into_iter().find_map(|x| match x.data {
        Tile::Label(b) => Some(*b),
        _ => None,
    })
}
fn space_neighbor(tile: &MapSquare<Tile>) -> Option<usize> {
    let neighbors = tile.neighbors();
    neighbors.into_iter().find_map(|x| match x.data {
        Tile::Space => Some(x.get_grid_index()),
        _ => None,
    })
}

fn get_portal_info(tile: &MapSquare<Tile>) -> Option<(char, char, usize)> {
    let &Tile::Label(a) = tile.data else { return None; };
    let Some(b) = label_neighbor(tile) else { return None; };
    let Some(grid_index) = space_neighbor(tile) else { return None; };

    Some((a, b, grid_index))
}

#[derive(Clone, Copy, Debug)]
struct Portal {
    grid_index: usize,
    outer: bool,
}

fn get_portals(map: &Map<Tile>) -> HashMap<(char, char), Vec<Portal>> {
    let mut portals: HashMap<(char, char), Vec<Portal>> = HashMap::new();
    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.get((x, y));
            if let Some((a, b, grid_index)) = get_portal_info(&tile) {
                // if we're at a portal, we need to find the space that's attached to it
                // make the key by sorting, since a and b won't always be in the same order
                let key = (a.min(b), a.max(b));
                let ends = portals.entry(key).or_default();

                // // figure out if the portal is on the inner ring or the outer ring
                let outer = y == 1 || x == 1 || y == map.height - 2 || x == map.width - 2;
                ends.push(Portal { grid_index, outer });
            }
        }
    }

    portals
}

fn get_edges(maze: &Map<Tile>) -> Vec<Vec<Edge>> {
    // Just construct a normal adjacency list, where walls, voids, and labels have no edges
    // we'll add in the portal edges later
    maze.into_iter()
        .map(|square| match square.data {
            Tile::Wall | Tile::Void => vec![],
            Tile::Label(_c) => vec![],
            _ => square
                .neighbors()
                .to_vec()
                .iter()
                .filter_map(|n| match n.data {
                    Tile::Wall | Tile::Void => None,
                    Tile::Label(_c) => None,
                    _ => Some(Edge::from_map_square(n)),
                })
                .collect(),
        })
        .collect()
}

fn problem1(input: &Input) -> usize {
    let m = Map::new(input.to_vec());
    m.print(|x| match x.data {
        Tile::Wall => '#',
        Tile::Space => '.',
        Tile::Void => ' ',
        Tile::Label(c) => *c,
    });

    let portals = get_portals(&m);
    let mut edges = get_edges(&m);

    // add the paired endpoints to the adjacency list
    for endpoints in portals.values() {
        if let &[e1, e2] = &endpoints[..] {
            edges[e1.grid_index].push(Edge::new(e2.grid_index));
            edges[e2.grid_index].push(Edge::new(e1.grid_index));
        }
    }

    let start = portals[&('A', 'A')].first().unwrap().grid_index;
    let end = portals[&('Z', 'Z')].first().unwrap().grid_index;

    shortest_path(&edges, start, end).unwrap()
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
        assert_eq!(result, 58)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
