use common::{
    answer,
    dijkstra::{shortest_path, Edge},
    grid::{Coord, Grid},
    nom::usize,
    read_input,
};
use nom::{
    character::complete::{char, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input, 70, 1024));
    answer!(problem2(&input, 70));
}

type Input = Vec<Coord>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_pair(usize, char(','), usize)).parse(input);

    result.unwrap().1
}

fn get_edges(grid: &Grid<bool>) -> Vec<Vec<Edge>> {
    grid.iter()
        .map(|square| {
            //corrupted squares have no edges
            if !square.data {
                return vec![];
            }

            square
                .neighbors()
                .iter()
                .filter(|&n| *n.data)
                .map(|n| Edge::from_map_square(n))
                .collect()
        })
        .collect()
}

fn problem1(input: &Input, size: usize, max: usize) -> usize {
    let mut grid = Grid::new(vec![vec![true; size + 1]; size + 1]);
    let start = grid.get_grid_index((0, 0));
    let goal = grid.get_grid_index((size, size));

    for &x in &input[0..max] {
        grid.set(x, false);
    }

    let edges = get_edges(&grid);
    let result = shortest_path(&edges, start, goal);

    result.unwrap()
}

fn problem2(input: &Input, size: usize) -> String {
    let mut grid = Grid::new(vec![vec![true; size + 1]; size + 1]);
    let start = grid.get_grid_index((0, 0));
    let goal = grid.get_grid_index((size, size));

    for &x in input {
        // let the byte fall
        grid.set(x, false);

        let edges = get_edges(&grid);
        if shortest_path(&edges, start, goal).is_none() {
            return format!("{},{}", x.0, x.1);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input, 6, 12);
        assert_eq!(result, 22)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input, 6);
        assert_eq!(result, "6,1".to_string())
    }
}
