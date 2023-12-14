use std::collections::{BTreeMap, VecDeque};

use common::extensions::vecvec::VecVec;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
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

type Input = Vec<Tile>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag("\n\n"),
        map(
            separated_pair(
                delimited(tag("Tile "), u64, tag(":")),
                newline,
                separated_list1(
                    newline,
                    many1(alt((map(char('#'), |_| true), map(char('.'), |_| false)))),
                ),
            ),
            |(id, v)| Tile::new(id, v),
        ),
    )(input);

    result.unwrap().1
}

type Edge = Vec<bool>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Tile {
    id: u64,
    value: Vec<Vec<bool>>,
}

impl Tile {
    fn new(id: u64, v: Vec<Vec<bool>>) -> Self {
        Self { id, value: v }
    }

    fn generate_translations(&self) -> Vec<Self> {
        let original = self.clone();
        let r90 = self.rotate();
        let r180 = r90.rotate();
        let r270 = r180.rotate();
        let flip = self.flip();
        let f90 = flip.rotate();
        let f180 = f90.rotate();
        let f270 = f180.rotate();

        vec![original, r90, r180, r270, flip, f90, f180, f270]
    }

    fn flip(&self) -> Self {
        Self {
            id: self.id,
            value: self.value.clone().into_iter().rev().collect(),
        }
    }

    fn rotate(&self) -> Self {
        Self {
            id: self.id,
            value: self.value.rotate(),
        }
    }

    fn top(&self) -> Edge {
        self.value.first().cloned().unwrap()
    }

    fn left(&self) -> Edge {
        self.value
            .iter()
            .filter_map(|r| r.first())
            .cloned()
            .collect()
    }

    fn bottom(&self) -> Edge {
        self.value.last().cloned().unwrap()
    }

    fn right(&self) -> Edge {
        self.value
            .iter()
            .filter_map(|r| r.last())
            .cloned()
            .collect()
    }

    fn get_edge_ids(&self) -> Vec<Edge> {
        let top = self.top();
        let bottom = self.bottom();
        let left = self.left();
        let right = self.right();
        [top, left, bottom, right]
            .iter()
            .map(Self::min_key)
            .collect()
    }

    fn inner(&self) -> Vec<Vec<bool>> {
        let max = self.value.len() - 1;
        (1..max)
            .map(|y| (1..max).map(|x| self.value[y][x]).collect())
            .collect()
    }

    fn min_key(e: &Edge) -> Edge {
        e.clone().min(e.iter().rev().cloned().collect())
    }
}

fn sort_tiles(input: &Input, edges: &BTreeMap<Edge, Vec<u64>>) -> Vec<(TileType, Tile)> {
    input
        .iter()
        .filter_map(|x| {
            let matching_edges = x
                .get_edge_ids()
                .iter()
                .filter(|e| edges[*e].len() == 2)
                .count();

            match matching_edges {
                2 => Some((TileType::Corner, x.clone())),
                3 => Some((TileType::Edge, x.clone())),
                _ => None,
            }
        })
        .collect()
}

#[derive(Debug)]
enum TileType {
    Corner,
    Edge,
}

fn create_edge_map(input: &Input) -> BTreeMap<Edge, Vec<u64>> {
    let mut edges: BTreeMap<Edge, Vec<u64>> = BTreeMap::new();
    for x in input {
        for e in x.get_edge_ids() {
            edges
                .entry(e)
                .and_modify(|v| v.push(x.id))
                .or_insert(vec![x.id]);
        }
    }

    edges
}

fn problem1(input: &Input) -> u64 {
    let board = assemble_board(input);

    let corners = [
        board.first().unwrap().first().unwrap().id,
        board.first().unwrap().last().unwrap().id,
        board.last().unwrap().first().unwrap().id,
        board.last().unwrap().last().unwrap().id,
    ];

    corners.iter().product()
}

enum Side {
    Top,
    Left,
    Bottom,
    Right,
}

fn assemble_board(input: &Input) -> Vec<Vec<Tile>> {
    let edges = create_edge_map(input);
    let tiles = sort_tiles(input, &edges);

    let side = (input.len() as f64).sqrt() as usize;
    let mut board: Vec<Vec<Option<Tile>>> = vec![vec![None; side]; side];
    let (_corner_type, corner_tile) = tiles
        .iter()
        .find(|x| matches!(x.0, TileType::Corner))
        .unwrap();

    let all_tiles: Vec<Tile> = input
        .iter()
        .flat_map(|x| x.generate_translations())
        .collect();

    // figure out the orientation of this corner tile, by finding the top & left that only have 1 match
    let corner: Vec<Tile> = corner_tile
        .generate_translations()
        .into_iter()
        .filter(|t| {
            let top = Tile::min_key(&t.top());
            let left = Tile::min_key(&t.left());
            let top = edges.get(&top).map(|x| x.len() == 1).unwrap_or_default();
            let left = edges.get(&left).map(|x| x.len() == 1).unwrap_or_default();

            top && left
        })
        .collect();
    let corner = corner.first().unwrap();
    board[0][0] = Some(corner.clone());

    // push the next tiles to look for on the queue
    let mut queue: VecDeque<(usize, usize, Side, Edge)> = VecDeque::new();
    queue.push_back((0, 1, Side::Left, corner.right()));
    queue.push_back((1, 0, Side::Top, corner.bottom()));

    while let Some((y, x, next_side, last_edge)) = queue.pop_front() {
        // have we already placed this tile?
        if board[y][x].is_some() {
            continue;
        }

        // figure out which IDs have already been placed
        let taken: Vec<u64> = board
            .iter()
            .flatten()
            .filter_map(|t| t.clone().map(|t| t.id))
            .collect();

        // find the next tile
        if let Some(next_tile) = all_tiles
            .iter()
            .filter(|t| !taken.contains(&t.id))
            .find(|t| match next_side {
                Side::Top => t.top() == last_edge,
                Side::Left => t.left() == last_edge,
                Side::Bottom => t.bottom() == last_edge,
                Side::Right => t.right() == last_edge,
            })
        {
            board[y][x] = Some(next_tile.clone());

            // queue up neighbors
            (y > 0).then(|| queue.push_back((y - 1, x, Side::Bottom, next_tile.top())));
            (y < side - 1).then(|| queue.push_back((y + 1, x, Side::Top, next_tile.bottom())));
            (x > 0).then(|| queue.push_back((y, x - 1, Side::Right, next_tile.left())));
            (x < side - 1).then(|| queue.push_back((y, x + 1, Side::Left, next_tile.right())));
        }
    }

    board
        .iter()
        .map(|r| r.iter().cloned().map(|c| c.unwrap()).collect())
        .collect()
}

fn flatten_image(board: &Vec<Vec<Tile>>) -> Tile {
    let side = board.len();
    // awful... vec vec vec vec
    let image: Vec<Vec<Vec<Vec<bool>>>> = board
        .iter()
        .map(|r| r.iter().map(|x| x.inner()).collect())
        .collect();

    // This is maybe the best way to compose a bunch of vectors together into a single grid
    let mut flattened_image = vec![];
    (0..side).for_each(|r| {
        (0..8).for_each(|y| {
            let mut row = vec![];
            (0..side).for_each(|c| {
                (0..8).for_each(|x| {
                    row.push(image[r][c][y][x]);
                });
            });
            flattened_image.push(row);
        });
    });

    Tile::new(0, flattened_image)
}

fn count_dragons(image: &Tile) -> usize {
    let side = image.value.len();

    let is_dragon = |y: usize, x: usize| {
        // make sure we don't walk off the edge of the image
        if y + 2 >= side || x + 19 >= side {
            return false;
        }

        // This is just...bad...but whatever. If all of these are true, then there's a dragon here
        let first_row = &[18];
        let second_row = &[0, 5, 6, 11, 12, 17, 18, 19];
        let third_row = &[1, 4, 7, 10, 13, 16];

        let first_matches = first_row.iter().all(|dx| image.value[y][x + dx]);
        let second_matches = second_row.iter().all(|dx| image.value[y + 1][x + dx]);
        let third_matches = third_row.iter().all(|dx| image.value[y + 2][x + dx]);

        first_matches && second_matches && third_matches
    };

    // find all the dragons
    let dragons = (0..side)
        .flat_map(|y| (0..side).map(move |x| (y, x)))
        .filter(|&(y, x)| is_dragon(y, x))
        .count();

    // there are 15 cells to a dragon
    dragons * 15
}

fn problem2(input: &Input) -> usize {
    // compose the image
    let board = assemble_board(input);
    let image = flatten_image(&board);

    // rotate the image and find the dragon count (only one translation will have dragons)
    let dragon_count = image
        .generate_translations()
        .iter()
        .map(count_dragons)
        .max()
        .unwrap();

    // count the rest of the hashes
    let total_hashes = image.value.iter().flatten().filter(|x| **x).count();
    total_hashes - dragon_count
}

#[cfg(test)]
mod test {
    use crate::{count_dragons, parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 20899048083289)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 273)
    }

    #[test]
    fn test_dragons() {
        let s = include_str!("../dragontest.txt");
        let tile = &parse(s)[0];
        let count = count_dragons(tile);
        assert_eq!(30, count);
    }
}
