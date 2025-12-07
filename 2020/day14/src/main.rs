use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
};

use common::{answer, read_input};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Instruction>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(
                preceded(
                    tag("mask = "),
                    many1(alt((
                        map(tag("1"), |_| MaskBit::One),
                        map(tag("0"), |_| MaskBit::Zero),
                        map(tag("X"), |_| MaskBit::X),
                    ))),
                ),
                |v| Instruction::Mask(Bitmask(v.into_iter().rev().collect())),
            ),
            map(
                separated_pair(delimited(tag("mem["), u64, tag("]")), tag(" = "), u64),
                |(idx, value)| Instruction::Set(Address36(idx), Value36(value)),
            ),
        )),
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
enum MaskBit {
    One,
    Zero,
    X,
}

#[derive(Clone, Debug)]
struct Bitmask(Vec<MaskBit>);

#[derive(Clone, Debug)]
enum Instruction {
    Mask(Bitmask),
    Set(Address36, Value36),
}

#[derive(Clone, Copy, Debug)]
struct Value36(u64);

impl Value36 {
    fn mask(&self, mask: &Bitmask) -> Self {
        let mut entry = 0;
        for k in 0..36 {
            let bit = match mask.0[k] {
                MaskBit::One => 1,
                MaskBit::Zero => 0,
                MaskBit::X => self.kth_bit(k),
            };

            entry |= bit << k;
        }

        Self(entry)
    }

    fn kth_bit(&self, k: usize) -> u64 {
        assert!(k < 36);
        (self.0 >> k) & 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Address36(u64);
impl Address36 {
    fn mask(&self, mask: &Bitmask) -> Vec<Address36> {
        let mut result = vec![];

        // we're gonna start from the 35th bit and count down because the test programs start with a bunch of zeros
        let mut queue: VecDeque<(usize, Address36)> = VecDeque::new();
        queue.push_back((35, Address36(0)));

        while let Some((k, current)) = queue.pop_front() {
            let bits: Vec<u64> = match mask.0[k] {
                // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
                MaskBit::One => vec![1],
                // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
                MaskBit::Zero => vec![self.kth_bit(k)],
                // If the bitmask bit is X, the corresponding memory address bit is floating.
                MaskBit::X => vec![0, 1],
            };

            for bit in bits {
                let new_address = Address36(current.0 | bit << k);
                // did we get to the 0th bit?
                if k == 0 {
                    result.push(new_address);
                } else {
                    // keep going
                    queue.push_back((k - 1, new_address));
                }
            }
        }

        result
    }

    fn kth_bit(&self, k: usize) -> u64 {
        assert!(k < 36);
        (self.0 >> k) & 1
    }
}

fn problem1(input: &Input) -> u64 {
    let mut mask = Bitmask(vec![]);
    let mut map: BTreeMap<Address36, Value36> = BTreeMap::new();
    for x in input {
        match x {
            Instruction::Mask(v) => mask = v.clone(),
            Instruction::Set(key, value) => {
                let result = value.mask(&mask);
                map.insert(*key, result);
            }
        }
    }

    map.values().map(|x| x.0).sum::<u64>()
}

fn problem2(input: &Input) -> u64 {
    let mut mask = Bitmask(vec![]);
    let mut map: BTreeMap<Address36, Value36> = BTreeMap::new();
    for x in input {
        match x {
            Instruction::Mask(v) => mask = v.clone(),
            Instruction::Set(key, value) => {
                for address in key.mask(&mask) {
                    map.insert(address, *value);
                }
            }
        }
    }

    map.values().map(|x| x.0).sum::<u64>()
}

impl Display for Bitmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.0.iter().rev() {
            match m {
                MaskBit::One => write!(f, "1")?,
                MaskBit::Zero => write!(f, "0")?,
                MaskBit::X => write!(f, "X")?,
            }
        }

        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mask(v) => {
                writeln!(f, "Setting mask")?;
                writeln!(f, "{v}")?;
            }
            Instruction::Set(_index, value) => {
                writeln!(f, "{value}")?;
            }
        };

        Ok(())
    }
}

impl Display for Value36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for k in (0..36).rev() {
            write!(f, "{:0b}", self.kth_bit(k))?;
        }
        write!(f, " (decimal {})", self.0)
    }
}

impl Display for Address36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for k in (0..36).rev() {
            write!(f, "{:0b}", self.kth_bit(k))?;
        }
        write!(f, " (decimal {})", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 165)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 208)
    }
}
