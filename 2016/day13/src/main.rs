use std::collections::HashSet;

use common::{
    dijkstra::{shortest_path, Edge},
    map::Map,
};

fn main() {
    let input = Input::new(1364, (31, 39));

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

struct Input {
    favorite_number: usize,
    x: usize,
    y: usize,
}

impl Input {
    fn new(favorite_number: usize, (x, y): (usize, usize)) -> Self {
        Input {
            favorite_number,
            x,
            y,
        }
    }
}

const PADDING: usize = 60;
fn generate_maze(input: &Input) -> Map<bool> {
    let max_x = PADDING;
    let max_y = PADDING;

    let mut result = vec![vec![false; max_x + 1]; max_y];
    (0..max_x).for_each(|x| {
        (0..max_y).for_each(|y| {
            let mut z = x * x + 3 * x + 2 * x * y + y + y * y;
            z += input.favorite_number;
            // odd is a wall
            result[y][x] = z.count_ones() % 2 != 0;
        });
    });

    Map::new(result)
}

fn print_maze(maze: &Map<bool>, seen: &HashSet<(usize, usize)>) {
    for (y, row) in maze.points.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                true => print!("#"),
                false if seen.contains(&(x, y)) => print!("O"),
                false => print!("."),
            }
        }
        println!();
    }
}

fn get_edges(maze: &Map<bool>) -> Vec<Vec<Edge>> {
    maze.into_iter()
        .map(|square| {
            // walls have no edges
            if *square.data {
                return vec![];
            }

            square
                .neighbors()
                .iter()
                .filter_map(|n| {
                    if *n.data {
                        return None;
                    }

                    Some(Edge {
                        node: n.get_grid_index(),
                        cost: 1,
                    })
                })
                .collect()
        })
        .collect()
}

fn problem1(input: &Input) -> usize {
    let maze = generate_maze(input);
    let edges = get_edges(&maze);

    shortest_path(
        &edges,
        maze.get((1, 1)).get_grid_index(),
        maze.get((input.x, input.y)).get_grid_index(),
    )
    .unwrap()
}

fn problem2(input: &Input) -> usize {
    let maze = generate_maze(input);
    let edges = get_edges(&maze);

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let start = maze.get((1, 1)).get_grid_index();

    for x in 0..PADDING {
        for y in (0..(PADDING - x)).rev() {
            if let Some(dist) = shortest_path(&edges, start, maze.get((x, y)).get_grid_index()) {
                if dist <= 50 {
                    seen.insert((x, y));
                }
            }
        }
    }

    print_maze(&maze, &seen);
    seen.len()
}

#[cfg(test)]
mod test {

    use crate::{problem1, problem2, Input};

    #[test]
    fn first() {
        let input = Input::new(10, (7, 4));
        let result = problem1(&input);
        assert_eq!(result, 11)
    }

    #[test]
    fn second() {
        let input = Input::new(10, (7, 4));
        let result = problem2(&input);
        assert_eq!(result, 151)
    }
}
