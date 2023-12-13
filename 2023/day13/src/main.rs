use std::fmt::Display;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Valley>;

fn parse(input: &str) -> Input {
    let image = map(
        separated_list1(
            newline,
            many1(alt((map(char('.'), |_| false), map(char('#'), |_| true)))),
        ),
        |x| Valley { valley: x },
    );
    let result: IResult<&str, Input> = separated_list1(tag("\n\n"), image)(input);

    result.unwrap().1
}

fn transpose(input: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = input[0].len();
    // transpose the nested vec so we can examine each char index
    let mut i_t: Vec<Vec<bool>> = vec![vec![]; width];
    (0..width).for_each(|x| {
        (0..input.len()).for_each(|y| i_t[x].push(input[y][x]));
    });

    i_t
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn to_score(self) -> usize {
        match self {
            Reflection::Horizontal(x) => 100 * x,
            Reflection::Vertical(x) => x,
        }
    }
}

struct Valley {
    valley: Vec<Vec<bool>>,
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.valley {
            for col in row {
                match col {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Valley {
    fn get_row_pair(&self, index: usize, offset: usize) -> Option<(&Vec<bool>, &Vec<bool>)> {
        let lower_idx = index.checked_sub(offset);
        let upper_idx = index.checked_add(offset + 1);

        let lower = lower_idx.and_then(|i| self.valley.get(i));
        let upper = upper_idx.and_then(|i| self.valley.get(i));

        lower.zip(upper)
    }

    fn find_reflections(&self) -> Vec<usize> {
        // find all the pairs that equal each other so we know where we have to start
        let top_reflections: Vec<usize> = self
            .valley
            .iter()
            .enumerate()
            .tuple_windows()
            .filter_map(|((a_idx, a), (_b_idx, b))| (a == b).then_some(a_idx))
            .collect();

        // for each reflection row we found, start from there and compare outward
        top_reflections
            .into_iter()
            .filter_map(|t| {
                // start at 1, we don't need to compare the rows we just compared
                let mut offset = 1;
                // search until there are no more pairs to compare because we walked off the end of the grid
                while let Some((upper, lower)) = self.get_row_pair(t, offset) {
                    // bail when we find something that doesn't match
                    if upper != lower {
                        return None;
                    }

                    // keep going
                    offset += 1;
                }

                Some(t + 1)
            })
            .collect_vec()
    }

    fn find_horizontal_reflection(&self) -> Vec<Reflection> {
        self.find_reflections()
            .into_iter()
            .map(Reflection::Horizontal)
            .collect_vec()
    }

    fn find_vertical_reflection(&self) -> Vec<Reflection> {
        let image = transpose(&self.valley);
        let v = Valley { valley: image };
        v.find_reflections()
            .into_iter()
            .map(Reflection::Vertical)
            .collect_vec()
    }

    fn find_reflection(&self) -> Reflection {
        let rs = self.find_all_reflections();
        rs.first().cloned().unwrap()
    }

    fn find_all_reflections(&self) -> Vec<Reflection> {
        vec![
            self.find_horizontal_reflection(),
            self.find_vertical_reflection(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    fn get_all_smudges(&self) -> Vec<(usize, usize, Valley)> {
        let mut all = vec![];
        for y in 0..self.valley.len() {
            for x in 0..self.valley[0].len() {
                let mut v = self.valley.clone();
                // swap the cell
                v[y][x] = !v[y][x];
                all.push((y, x, Valley { valley: v }));
            }
        }

        all
    }
}

fn problem1(input: &Input) -> usize {
    input.iter().map(|v| v.find_reflection().to_score()).sum()
}

fn problem2(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|v| {
            let r = v.find_reflection();
            let smudges = v.get_all_smudges();
            let smudge_reflections: Vec<Reflection> = smudges
                .iter()
                .flat_map(|(_, _, v1)| v1.find_all_reflections())
                .filter(|r1| r != *r1)
                .unique()
                .collect_vec();
            smudge_reflections
        })
        .map(|x| x.to_score())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn horizontal() {
        let input = include_str!("../horizontal.txt");
        let input = parse(input);
        let result = input[0]
            .find_horizontal_reflection()
            .first()
            .unwrap()
            .to_score();
        assert_eq!(result, 400)
    }

    #[test]
    fn vertical() {
        let input = include_str!("../vertical.txt");
        let input = parse(input);
        let result = input[0]
            .find_vertical_reflection()
            .first()
            .unwrap()
            .to_score();
        assert_eq!(result, 5)
    }

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 405)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 400)
    }
}
