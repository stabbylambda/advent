use std::collections::BTreeMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, u32 as nom_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::{Input, Valve};
fn parse_valve(input: &str) -> IResult<&str, (&str, u32)> {
    separated_pair(
        preceded(tag("Valve "), alpha1),
        tag(" has flow rate="),
        nom_u32,
    )(input)
}

fn parse_adjacent(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        alt((
            tag("tunnels lead to valves "),
            tag("tunnel leads to valve "),
        )),
        separated_list1(tag(", "), alpha1),
    )(input)
}

pub(crate) fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            map(
                separated_pair(parse_valve, tag("; "), parse_adjacent),
                |((name, flow_rate), neighbors)| (name, flow_rate, neighbors),
            ),
        ),
        |valves| {
            let indexes: BTreeMap<&str, usize> = valves
                .iter()
                .enumerate()
                .map(|(idx, (name, _, _))| (*name, idx))
                .collect();

            let valves: Vec<Valve> = valves
                .iter()
                .enumerate()
                .map(|(id, (_name, flow_rate, neighbor_strings))| {
                    let neighbors = neighbor_strings.iter().map(|n| indexes[n]).collect();
                    Valve {
                        id,
                        flow_rate: *flow_rate,
                        neighbors,
                    }
                })
                .collect();

            crate::Caves::new(valves, indexes["AA"])
        },
    )(input);

    result.unwrap().1
}
