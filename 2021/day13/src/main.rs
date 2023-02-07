use std::{
    fmt::{Debug, Display},
    vec,
};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
fn main() {
    let lines = include_str!("../input.txt");

    let score = problem1(lines);
    println!("problem 1 score: {score}");

    let score = problem2(lines);
    println!("problem 2 score:\n{score}");
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(coords: Vec<(u32, u32)>) -> Grid {
        let x_max = coords.iter().map(|x| x.0).max().unwrap() as usize;
        let y_max = coords.iter().map(|x| x.1).max().unwrap() as usize;
        let mut grid = vec![vec![false; x_max + 1]; y_max + 2];

        for (x, y) in coords {
            grid[y as usize][x as usize] = true;
        }

        Grid { grid }
    }

    fn count(&self) -> u32 {
        self.grid.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|x| **x == true).count() as u32
        })
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid.first().unwrap().len()
    }

    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(n) => self.fold_x(*n),
            Fold::Y(n) => self.fold_y(*n),
        };
    }

    fn fold_x(&mut self, n: usize) {
        let mut result = vec![vec![false; n]; self.height()];

        for y in 0..self.height() {
            for x in 0..n {
                let mirror_x = self.width() - x - 1;
                result[y][x] = self.grid[y][x] || self.grid[y][mirror_x];
            }
        }

        self.grid = result;
    }

    fn fold_y(&mut self, n: usize) {
        let mut result = vec![vec![false; self.width()]; n];

        for y in 0..n {
            for x in 0..self.width() {
                let mirror_y = self.height() - y - 2;
                result[y][x] = self.grid[y][x] || self.grid[mirror_y][x];
            }
        }

        self.grid = result;
    }
}
struct Input {
    grid: Grid,
    folds: Vec<Fold>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{self}")
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                let x = match col {
                    true => "#",
                    false => ".",
                };
                write!(f, "{x}")?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn parse(s: &str) -> IResult<&str, Input> {
    map(
        separated_pair(
            separated_list1(line_ending, separated_pair(nom_u32, tag(","), nom_u32)),
            tag("\n\n"),
            separated_list1(
                line_ending,
                preceded(
                    tag("fold along "),
                    separated_pair(one_of("xy"), tag("="), nom_u32),
                ),
            ),
        ),
        |(coords, folds)| {
            let grid = Grid::new(coords);
            let folds = folds
                .iter()
                .map(|f| match f {
                    ('x', num) => Fold::X(*num as usize),
                    ('y', num) => Fold::Y(*num as usize),
                    _ => panic!(),
                })
                .collect();
            Input { grid, folds }
        },
    )(s)
}

fn problem1(lines: &str) -> u32 {
    let Input { mut grid, folds } = parse(lines).unwrap().1;
    grid.fold(&folds[0]);

    grid.count()
}

fn problem2(lines: &str) -> String {
    let Input { mut grid, folds } = parse(lines).unwrap().1;
    for fold in folds {
        grid.fold(&fold);
    }
    grid.to_string()
}

#[cfg(test)]
mod test {

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let lines = include_str!("../test.txt");
        let result = problem1(&lines);
        assert_eq!(result, 17)
    }

    #[test]
    fn second() {
        let lines = include_str!("../test.txt");
        let result = problem2(&lines);
        let expected = r#"#####
#...#
#...#
#...#
#####
.....
.....
"#;
        assert_eq!(result, expected)
    }
}
