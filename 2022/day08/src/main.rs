use common::{
    grid::{orthogonal::Orthogonal, Grid, GridSquare},
    nom::{parse_grid, single_digit},
};
use nom::IResult;

fn main() {
    let lines = include_str!("../test.txt");
    let input = parse(lines);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Tree = u32;

fn parse(lines: &str) -> Grid<Tree> {
    let parsed: IResult<&str, Grid<Tree>> = parse_grid(single_digit)(lines);

    parsed.unwrap().1
}

fn problem1(map: &Grid<Tree>) -> u32 {
    map.iter().fold(0, |acc, square| {
        let neighbors = map.orthogonal_neighbors(&square);
        let tree = square.data;

        // check the vertical and horizontal from this tree
        let visible_from_north = neighbors.north.iter().all(|h| h.data < tree);
        let visible_from_south = neighbors.south.iter().all(|h| h.data < tree);
        let visible_from_west = neighbors.west.iter().all(|h| h.data < tree);
        let visible_from_east = neighbors.east.iter().all(|h| h.data < tree);

        let is_visible =
            visible_from_north || visible_from_south || visible_from_east || visible_from_west;

        acc + is_visible as u32
    })
}

fn view<'a>(height: &'a Tree, neighbors: Vec<GridSquare<'a, Tree>>) -> u32 {
    let mut view = 0;
    for h in neighbors {
        view += 1;
        if h.data >= height {
            break;
        }
    }

    view
}

fn problem2(map: &Grid<Tree>) -> u32 {
    map.iter()
        .map(|square| {
            let tree = square.data;
            let neighbors = map.orthogonal_neighbors(&square);

            let north = view(tree, neighbors.north);
            let south = view(tree, neighbors.south);
            let east = view(tree, neighbors.east);
            let west = view(tree, neighbors.west);

            north * south * east * west
        })
        .max()
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
        assert_eq!(result, 21)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 8)
    }
}
