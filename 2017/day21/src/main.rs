use std::collections::HashMap;

use common::extensions::vecvec::VecVec;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input = Vec<(Pattern, Pattern)>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pattern {
    value: Vec<Vec<bool>>,
}
impl Pattern {
    fn new(s: &str) -> Pattern {
        let v = s
            .split('/')
            .map(|x| x.chars().map(|c| c == '#').collect())
            .collect();

        Pattern { value: v }
    }

    fn generate_translations(&self) -> Vec<Pattern> {
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

    fn flip(&self) -> Pattern {
        Pattern {
            value: self.value.clone().into_iter().rev().collect(),
        }
    }

    fn rotate(&self) -> Pattern {
        Pattern {
            value: self.value.rotate(),
        }
    }

    fn size(&self) -> usize {
        self.value.len()
    }

    fn pixels_on(&self) -> usize {
        self.value
            .iter()
            .map(|x| x.iter().filter(|x| **x).count())
            .sum()
    }

    fn divide(&self) -> Vec<Vec<Pattern>> {
        let size = self.size();
        let step = if size.is_multiple_of(2) { 2 } else { 3 };

        // I guarantee there's a nicer way to do this...but I can't think of it right now
        (0..size)
            .step_by(step)
            .map(|y| {
                (0..size)
                    .step_by(step)
                    .map(|x| {
                        let v = &self.value;
                        let value = if step == 2 {
                            vec![
                                vec![v[y][x], v[y][x + 1]],
                                vec![v[y + 1][x], v[y + 1][x + 1]],
                            ]
                        } else {
                            {
                                vec![
                                    vec![v[y][x], v[y][x + 1], v[y][x + 2]],
                                    vec![v[y + 1][x], v[y + 1][x + 1], v[y + 1][x + 2]],
                                    vec![v[y + 2][x], v[y + 2][x + 1], v[y + 2][x + 2]],
                                ]
                            }
                        };

                        Pattern { value }
                    })
                    .collect()
            })
            .collect()
    }

    /* TODO: the trick here would be noticing the repeating pattern of the fractal and keeping
    track of generations to do a multiplication when counting the pixels_on. But this works today and isn't
    incredibly slow.
    */
    fn apply(&self, all_rules: &HashMap<Pattern, Pattern>) -> Pattern {
        // divide and apply rules
        let mapped: Vec<Vec<Pattern>> = self
            .divide()
            .into_iter()
            .map(|y| y.into_iter().map(|x| all_rules[&x].clone()).collect())
            .collect();

        // get the size of the new grid
        let inner_size = mapped[0][0].size();
        let outer_size = mapped.len();
        let size = outer_size * inner_size;

        // recombine everything into a single pattern
        let mut value = vec![vec![false; size]; size];
        for (py, prow) in mapped.iter().enumerate() {
            for (px, pattern) in prow.iter().enumerate() {
                for (y, row) in pattern.value.iter().enumerate() {
                    for (x, cell) in row.iter().enumerate() {
                        value[py * inner_size + y][px * inner_size + x] = *cell;
                    }
                }
            }
        }

        Pattern { value }
    }
}

fn parse(input: &str) -> Input {
    let pattern = |s| map(take_till(|x| x == ' ' || x == '\n'), Pattern::new).parse(s);
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_pair(pattern, tag(" => "), pattern)).parse(input);

    result.unwrap().1
}

fn problem(input: &Input) -> (usize, usize) {
    // generate all the rules based off the existing rules plus rotations / translations
    let all_rules: HashMap<Pattern, Pattern> = input
        .iter()
        .flat_map(|(k, v)| {
            k.generate_translations()
                .into_iter()
                .map(|k| (k, v.clone()))
                .collect::<Vec<(Pattern, Pattern)>>()
        })
        .collect();

    // this is the starting pattern the program uses
    let pattern = Pattern::new(".#./..#/###");

    // run 18 generations of iteration and keep track of all the pixel counts
    let (on, pattern) = (0..18).fold((vec![], pattern), |(mut acc, pattern), _x| {
        let new_pattern = pattern.apply(&all_rules);
        acc.push(new_pattern.pixels_on());
        (acc, new_pattern)
    });

    // we only care about the 5th generation and the last
    (on[4], pattern.pixels_on())
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (on5, on18) = problem(&input);
        assert_eq!(on5, 203);
        assert_eq!(on18, 3342470);
    }
}
