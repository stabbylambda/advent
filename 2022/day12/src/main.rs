use common::{answer, read_input};
use common::dijkstra::{shortest_path, Edge};
use common::grid::Grid;
use common::nom::parse_grid;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::none_of, combinator::map, IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

#[derive(Clone, Copy, Debug)]
enum Position {
    Start,
    End,
    Normal(char),
}

impl From<&Position> for u32 {
    fn from(val: &Position) -> Self {
        (match val {
            Position::Start => 'a',
            Position::End => 'z',
            Position::Normal(c) => *c,
        }) as u32
    }
}

impl Position {
    fn can_travel_to(&self, dest: &Position) -> bool {
        let start_height: u32 = self.into();
        let dest_height: u32 = dest.into();

        dest_height <= start_height + 1
    }

    fn is_potential_start(&self) -> bool {
        matches!(self, Position::Start | Position::Normal('a'))
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Grid<Position>> = parse_grid(alt((
        map(tag("S"), |_| Position::Start),
        map(tag("E"), |_| Position::End),
        map(none_of("\n"), Position::Normal),
    ))).parse(input);

    result.unwrap().1
}

type Input = Grid<Position>;

fn get_edges(map: &Grid<Position>) -> Vec<Vec<Edge>> {
    map.iter()
        .map(|square| {
            square
                .neighbors()
                .iter()
                .filter(|&neighbor| square.data.can_travel_to(neighbor.data))
                .map(|neighbor| Edge::from_map_square(neighbor))
                .collect()
        })
        .collect()
}

fn problem1(map: &Input) -> usize {
    let mut start: usize = 0;
    let mut finish: usize = 0;

    // find both the start and finish squares
    for square in map.iter() {
        match square.data {
            Position::Start => start = square.get_grid_index(),
            Position::End => finish = square.get_grid_index(),
            Position::Normal(_) => {}
        }
    }
    let edges = get_edges(map);
    shortest_path(&edges, start, finish).unwrap()
}

fn problem2(map: &Input) -> usize {
    // find the only finish square
    let mut finish: usize = 0;
    for square in map.iter() {
        if let Position::End = square.data {
            finish = square.get_grid_index()
        }
    }

    let edges = get_edges(map);

    map.iter()
        // only take the potential starting locations
        .filter(|s| s.data.is_potential_start())
        // find the shortest paths from a to z
        .filter_map(|start| shortest_path(&edges, start.get_grid_index(), finish))
        // get the shortest
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 31)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 29)
    }
}
