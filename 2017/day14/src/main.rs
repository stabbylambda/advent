use advent_2017_10::hash_str;
use common::{
    dijkstra::{connected_components, Edge},
    grid::Grid,
};

fn main() {
    let input = include_str!("../input.txt");

    let answer = problem1(input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input);
    println!("problem 2 answer: {answer}");
}

fn problem1(input: &str) -> u32 {
    (0..128)
        .map(|n| hash_str(&format!("{input}-{n}")))
        .map(|x| x.count_ones())
        .sum()
}

fn get_edges(maze: &Grid<bool>) -> Vec<Vec<Edge>> {
    maze.iter()
        .map(|square| {
            // empty spaces have no edges
            if !square.data {
                return vec![];
            }

            square
                .neighbors()
                .iter()
                .filter_map(|n| {
                    if !n.data {
                        return None;
                    }

                    Some(Edge::from_map_square(n))
                })
                .collect()
        })
        .collect()
}

fn problem2(input: &str) -> usize {
    // we need to take the input, hash each row, and then get the bits
    let grid: Vec<Vec<bool>> = (0..128)
        .map(|n| hash_str(&format!("{input}-{n}")))
        .map(|x| (0..128).map(|n| (x >> (127 - n) & 1) != 0).collect())
        .collect();

    // now we need to get the connected components in the graph
    let grid = Grid::new(grid);
    let edges = get_edges(&grid);
    let connected = connected_components(&edges);

    // for each connected component, we need to check if it's a used square
    connected
        .into_keys()
        .filter(|k| *grid.get_from_grid_index(*k).data)
        .count()
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let result = problem1(input);
        assert_eq!(result, 8108)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let result = problem2(input);
        assert_eq!(result, 1242)
    }
}
