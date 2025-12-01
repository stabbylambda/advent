use common::{
    dijkstra::{shortest_path, Edge},
    grid::Grid,
    nom::{parse_grid, single_digit},
};
use nom::{combinator::map, IResult, Parser};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

#[derive(Debug)]
struct Input {
    start: usize,
    finish: usize,
    map: Grid<usize>,
}

impl Input {
    fn new(map: Grid<usize>) -> Input {
        let start = map.get((0, 0)).get_grid_index();
        let finish = map.get((map.width - 1, map.height - 1)).get_grid_index();

        Input { map, start, finish }
    }
}

fn get_edges(map: &Grid<usize>) -> Vec<Vec<Edge>> {
    map.iter()
        .map(|square| {
            square
                .neighbors()
                .iter()
                .map(|n| Edge {
                    node: n.get_grid_index(),
                    cost: *n.data,
                })
                .collect()
        })
        .collect()
}

fn parse(input: &str) -> Input {
    let weights: IResult<&str, Input> =
        map(parse_grid(map(single_digit, |x| x as usize)), Input::new).parse(input);

    weights.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let edges = get_edges(&input.map);
    shortest_path(&edges, input.start, input.finish).unwrap()
}

fn multiply_map(map: &Grid<usize>) -> Grid<usize> {
    let mut v = vec![vec![0; map.height * 5]; map.width * 5];
    for mx in 0..5 {
        for my in 0..5 {
            for x in 0..map.width {
                for y in 0..map.height {
                    let adj = match (mx + my + map.get((x, y)).data) % 9 {
                        0 => 9,
                        n => n,
                    };
                    v[my * map.height + y][mx * map.width + x] = adj;
                }
            }
        }
    }
    Grid::new(v)
}

fn problem2(input: &Input) -> usize {
    let result = multiply_map(&input.map);
    let input = Input::new(result);
    let edges = get_edges(&input.map);
    shortest_path(&edges, input.start, input.finish).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{multiply_map, parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 40)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 315)
    }

    #[test]
    fn test_mul_easy() {
        let input = parse("8");

        dbg!(multiply_map(&input.map));
    }

    #[test]
    fn test_mul() {
        let expected = include_str!("../expected.txt");
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = multiply_map(&input.map);

        let expected = parse(expected);

        for x in 0..result.width {
            for y in 0..result.height {
                assert_eq!(expected.map.points[y][x], result.points[y][x])
            }
        }
    }
}
