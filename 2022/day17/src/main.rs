use std::{
    collections::HashMap,
    fmt::Display,
    ops::{BitAnd, BitOrAssign, Range},
};

use bitvec::{macros::internal::funty::Fundamental, prelude::*};

use nom::{character::complete::anychar, combinator::map, multi::many1, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Jet>;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Jet {
    Left,
    Right,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(map(anychar, |x| match x {
        '<' => Jet::Left,
        '>' => Jet::Right,
        _ => unreachable!(),
    }))(input);

    result.unwrap().1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockKind {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}

impl From<RockKind> for Rock {
    fn from(val: RockKind) -> Self {
        let rock = match val {
            // I could calculate these as hex values, but I like the visual representation
            RockKind::Horizontal => bitvec![u8, Lsb0;
                0,0,1,1,1,1,0,
                0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,
            ],
            RockKind::Plus => bitvec![u8, Lsb0;
                0,0,0,1,0,0,0,
                0,0,1,1,1,0,0,
                0,0,0,1,0,0,0,
                0,0,0,0,0,0,0,
            ],
            RockKind::L => bitvec![u8, Lsb0;
                0,0,1,1,1,0,0,
                0,0,0,0,1,0,0,
                0,0,0,0,1,0,0,
                0,0,0,0,0,0,0,
            ],
            RockKind::Vertical => bitvec![u8, Lsb0;
                0,0,1,0,0,0,0,
                0,0,1,0,0,0,0,
                0,0,1,0,0,0,0,
                0,0,1,0,0,0,0,
            ],
            RockKind::Square => bitvec![u8, Lsb0;
                0,0,1,1,0,0,0,
                0,0,1,1,0,0,0,
                0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,
            ],
        };

        Rock { bits: rock }
    }
}

impl RockKind {
    fn all_kinds() -> Vec<RockKind> {
        vec![
            Self::Horizontal,
            Self::Plus,
            Self::L,
            Self::Vertical,
            Self::Square,
        ]
    }
}
struct Rock {
    bits: BitVec<u8>,
}

impl Rock {
    fn blow(&mut self, jet: &Jet, tower: &Tower, height: usize) {
        let left_wall = bitvec![u8, Lsb0;
                1,0,0,0,0,0,0,
                1,0,0,0,0,0,0,
                1,0,0,0,0,0,0,
                1,0,0,0,0,0,0,
        ];
        let right_wall = bitvec![u8, Lsb0;
                0,0,0,0,0,0,1,
                0,0,0,0,0,0,1,
                0,0,0,0,0,0,1,
                0,0,0,0,0,0,1,
        ];

        let on_left_wall = self.bits.clone().bitand(left_wall).any();
        let on_right_wall = self.bits.clone().bitand(right_wall).any();

        let mut new_bits = self.bits.clone();
        match jet {
            Jet::Left if !on_left_wall => &new_bits.shift_left(1),
            Jet::Right if !on_right_wall => &new_bits.shift_right(1),
            _ => {
                return;
            }
        };

        let rock_collision = tower.collision(&new_bits, height);
        if rock_collision {
            return;
        }

        self.bits = new_bits;
    }
}
impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in (0..4).rev() {
            let range = (n * Tower::WIDTH)..(n * Tower::WIDTH + 7);
            let x = &self.bits[range];
            write!(f, "|")?;
            for y in x {
                let y = if y.as_bool() { "." } else { "#" };
                write!(f, "{y}")?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f)
    }
}

struct Tower {
    bits: BitVec<u8>,
}

impl Tower {
    const WIDTH: usize = 7;

    fn new() -> Tower {
        // put a solid floor at the bottom of the tower
        let b = bitvec![u8, Lsb0;];
        Tower::from_bits(b)
    }

    fn from_bits(bits: BitVec<u8>) -> Tower {
        Tower { bits }
    }

    fn grow(&mut self) -> usize {
        let b = bitvec![u8, Lsb0; 0; 4*Tower::WIDTH];
        self.bits.extend(b);
        self.get_height() + 3
    }

    fn get_height(&self) -> usize {
        (self.bits.len() - self.bits.trailing_zeros()).div_ceil(Tower::WIDTH)
    }

    fn get_row_range(&self, row: usize) -> Range<usize> {
        let start_index = row * Tower::WIDTH;
        let end_index = start_index + Tower::WIDTH;

        start_index..end_index
    }

    fn get_tower_slice(&self, row: usize) -> Range<usize> {
        let start_index = row * Tower::WIDTH;
        let end_index = self.bits.len().min(start_index + (4 * Tower::WIDTH));

        start_index..end_index
    }

    fn collision(&self, rock: &BitSlice<u8>, height: usize) -> bool {
        let r = self.get_tower_slice(height);
        let result = self.bits[r].to_bitvec().bitand(rock);

        result.any()
    }

    fn merge(&mut self, rock: &mut Rock, height: usize) {
        let r = self.get_tower_slice(height);

        self.bits[r].bitor_assign(&rock.bits);
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..=self.get_height()).rev() {
            let r = self.get_row_range(row);
            let x = &self.bits[r];
            write!(f, "{row:3} |")?;
            for y in x {
                let y = if y.as_bool() { "." } else { "#" };
                write!(f, "{y}")?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "Tower height = {}", self.get_height())
    }
}

fn problem1(input: &Input) -> usize {
    problem(input, 2022)
}
fn problem(input: &Input, limit: usize) -> usize {
    let mut checkpoints: HashMap<(usize, usize, u8), (usize, usize)> = HashMap::new();

    let mut rocks = RockKind::all_kinds().into_iter().enumerate().cycle();
    let mut jets = input.iter().cycle();
    let mut tower = Tower::new();
    let mut drop_count = 0;
    let mut jet_index = 0;
    let mut skipped_height = None;

    while drop_count < limit {
        let (rock_index, kind) = rocks.next().unwrap();
        let mut rock: Rock = kind.into();
        let mut height = tower.grow();
        let original_height = height;

        loop {
            // blow the rock one way or another first
            jet_index = (jet_index + 1) % input.len();
            let jet = jets.next().unwrap();

            rock.blow(jet, &tower, height);

            if height == 0 || tower.collision(&rock.bits, height - 1) {
                tower.merge(&mut rock, height);
                drop_count += 1;
                if original_height > 1000 {
                    let top = &tower.bits[tower.get_row_range(tower.get_height() - 1)];
                    let top: u8 = top.load();
                    if let Some((prev_height, prev_drops)) = checkpoints.insert(
                        (rock_index, jet_index, top),
                        (tower.get_height(), drop_count),
                    ) {
                        let cycle_size = prev_drops.abs_diff(drop_count);
                        let skip_count = (limit - drop_count) / cycle_size;
                        let skipped_drops = skip_count * cycle_size;

                        let cycle_height = prev_height.abs_diff(tower.get_height());
                        skipped_height = Some(skip_count * cycle_height);

                        println!(
                            "found cycle on drop {drop_count}. Skipping {skip_count} cycles of {cycle_size} size for {skipped_drops}"
                        );

                        drop_count += skipped_drops;

                        // don't let it find a new cycle on the next line, just blow away everything
                        checkpoints.clear();
                    }
                }
                break;
            }

            height -= 1;
        }
    }

    tower.get_height() + skipped_height.unwrap_or(0)
}

fn problem2(input: &Input) -> usize {
    problem(input, 1_000_000_000_000)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    #[ignore]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 3068)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1514285714288)
    }
}
