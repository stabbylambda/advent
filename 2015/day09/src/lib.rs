use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{algo::all_simple_paths, prelude::UnGraphMap};

pub type Input<'a> = Vec<Route<'a>>;

#[derive(Debug)]
pub struct Route<'a> {
    from: &'a str,
    to: &'a str,
    distance: u32,
}

pub fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                separated_pair(alpha0, tag(" to "), alpha0),
                tag(" = "),
                nom_u32,
            ),
            |((from, to), distance)| Route { from, to, distance },
        ),
    )(input);

    result.unwrap().1
}

pub fn get_graph<'a>(input: &'a Input) -> UnGraphMap<&'a str, u32> {
    let mut g: UnGraphMap<&str, u32> = UnGraphMap::new();
    for route in input {
        g.add_node(route.from);
        g.add_node(route.to);
        g.add_edge(route.from, route.to, route.distance);
    }

    g
}

pub fn problem1(input: &Input) -> (u32, u32) {
    let g = get_graph(input);
    let path_length = g.node_count() - 2;

    let MinMaxResult::MinMax(min, max) = g
        .nodes()
        .tuple_combinations()
        .flat_map(|(start, end)| {
            all_simple_paths::<Vec<_>, _>(&g, start, end, path_length, Some(path_length))
        })
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(x, y)| g.edge_weight(x, y).unwrap())
                .sum::<u32>()
        })
        .minmax()
    else {
        panic!()
    };

    (min, max)
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn problem() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (min, max) = problem1(&input);
        assert_eq!(min, 605);
        assert_eq!(max, 982);
    }
}
