use common::{answer, read_input};
use common::nom::usize;
use ndarray::{prelude::*, OwnedRepr};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list0,
    sequence::terminated,
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<(usize, usize, usize)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Vec<(usize, usize, usize)>> = separated_list0(
        newline,
        (
            terminated(usize, tag(",")),
            terminated(usize, tag(",")),
            usize,
        ),
    )
    .parse(input);

    result.unwrap().1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Lava,
    Vacuum,
}

struct Grid {
    grid: ArrayBase<OwnedRepr<Cell>, Dim<[usize; 3]>>,
}

impl From<&Vec<(usize, usize, usize)>> for Grid {
    fn from(value: &Vec<(usize, usize, usize)>) -> Self {
        Grid::from_points(value)
    }
}

impl Grid {
    const SIZE: usize = 24;
    fn from_points(points: &[(usize, usize, usize)]) -> Grid {
        let mut grid = Array3::<Cell>::from_elem((Self::SIZE, Self::SIZE, Self::SIZE), Cell::Air);
        for &(x, y, z) in points {
            // push them up by one so we can get the edges correctly
            grid[[x + 1, y + 1, z + 1]] = Cell::Lava;
        }

        Grid { grid }
    }

    // leave a ring of air around the grid
    fn set_vacuum(&mut self) {
        for x in 1..Self::SIZE {
            for y in 1..Self::SIZE {
                for z in 1..Self::SIZE {
                    if self.grid[[x, y, z]] == Cell::Air {
                        self.grid[[x, y, z]] = Cell::Vacuum;
                    }
                }
            }
        }
    }
    fn get_surface_area(&self) -> usize {
        let mut total_empty = 0;
        for x in 1..Self::SIZE {
            for y in 1..Self::SIZE {
                for z in 1..Self::SIZE {
                    if self.grid[[x, y, z]] == Cell::Lava {
                        // this is a voxel, check all its neighbors
                        total_empty += [
                            [x + 1, y, z],
                            [x - 1, y, z],
                            [x, y + 1, z],
                            [x, y - 1, z],
                            [x, y, z + 1],
                            [x, y, z - 1],
                        ]
                        .iter()
                        .filter(|&&p| self.grid[p] == Cell::Air)
                        .count();
                    }
                }
            }
        }
        total_empty
    }

    fn flood_fill(&mut self) {
        self.set_vacuum();
        let mut updated = true;
        while updated {
            updated = false;
            for x in 1..Self::SIZE - 1 {
                for y in 1..Self::SIZE - 1 {
                    for z in 1..Self::SIZE - 1 {
                        // check if this cell is a vacuum and any of its neighbors are air
                        if self.grid[[x, y, z]] == Cell::Vacuum {
                            let any_air_neighbors = [
                                [x + 1, y, z],
                                [x - 1, y, z],
                                [x, y + 1, z],
                                [x, y - 1, z],
                                [x, y, z + 1],
                                [x, y, z - 1],
                            ]
                            .iter()
                            .any(|&p| self.grid[p] == Cell::Air);

                            if any_air_neighbors {
                                updated = true;
                                self.grid[[x, y, z]] = Cell::Air;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn problem1(input: &Input) -> usize {
    let grid: Grid = input.into();
    grid.get_surface_area()
}

fn problem2(input: &Input) -> usize {
    let mut grid: Grid = input.into();
    grid.flood_fill();
    grid.get_surface_area()
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn supermini() {
        let v = vec![(1, 1, 1)];
        let result = problem1(&v);
        assert_eq!(result, 6)
    }

    #[test]
    fn mini() {
        let v = vec![(1, 1, 1), (2, 1, 1)];
        let result = problem1(&v);
        assert_eq!(result, 10)
    }
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 64)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 58)
    }
}
