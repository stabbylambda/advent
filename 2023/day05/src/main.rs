use std::{collections::VecDeque, ops::Range};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Almanac;

fn parse(input: &str) -> Input {
    let almanac_range = map(separated_list1(tag(" "), u64), |v| {
        AlmanacRange::new(v[0], v[1], v[2])
    });

    let almanac_maps = separated_list1(
        tag("\n\n"),
        map(
            (
                terminated(take_until(" "), tag(" map:\n")),
                separated_list1(newline, almanac_range),
            ),
            |(_name, ranges)| AlmanacMap { ranges },
        ),
    );

    let seeds = preceded(tag("seeds: "), separated_list1(tag(" "), u64));

    let result: IResult<&str, Input> = map(
        separated_pair(seeds, tag("\n\n"), almanac_maps),
        |(seeds, maps)| Almanac { seeds, maps },
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
struct AlmanacRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacRange {
    fn new(destination_start: u64, source_start: u64, length: u64) -> Self {
        Self {
            destination_start,
            source_start,
            length,
        }
    }

    fn get_overlap(&self, r: &Range<u64>) -> Option<Range<u64>> {
        let overlap_start = r.start.max(self.source_start);
        let overlap_end = r.end.min(self.source_start + self.length);

        let overlap = overlap_start..overlap_end;
        (!overlap.is_empty()).then_some(overlap)
    }

    fn create_mapped_range(&self, overlap: Range<u64>) -> Range<u64> {
        // Get the offsets
        let start_offset = overlap.start - self.source_start;
        let end_offset = overlap.end - self.source_start;

        // Map the range
        let new_start = self.destination_start + start_offset;
        let new_end = self.destination_start + end_offset;

        new_start..new_end
    }
}

#[derive(Clone, Debug)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn process(&self, seeds: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut new_seeds = vec![];

        /* I would normally write this up as a fold, but we have to consider partial range overlaps, which means
        we'll need to break the AlmanacRange in two and then re-consider the remainder of the range. Consider the
        situation where a seed range overlaps 2 ranges:

        Seed Range:                       | ---- sr1 ---- |
        Almanac Ranges:        | ---- ar1 ---- |    | ---- ar2 ----- |

        We'll have to break sr1 into three chunks. Rather than having a huge else if, it's easier to just consider
        each overlapping segment and then push back the remainders onto the VecDeque. This would get worse if we
        have an interior almanac range between ar1 and ar2 (which does happen in the real input).
        */
        let mut seed_queue = VecDeque::from(seeds);
        while let Some(range) = seed_queue.pop_front() {
            let mapped: Vec<Range<u64>> = self
                .ranges
                .iter()
                .filter_map(|almanac_range| {
                    almanac_range.get_overlap(&range).map(|overlap| {
                        // if we're not fully overlapped, then we need to push partial ranges back to consider again
                        if overlap.start > range.start {
                            seed_queue.push_back(range.start..overlap.start)
                        }

                        if overlap.end < range.end {
                            seed_queue.push_back(overlap.end..range.end)
                        }

                        // map the overlapped part of the range
                        almanac_range.create_mapped_range(overlap)
                    })
                })
                .collect();

            if mapped.is_empty() {
                // nothing got mapped, so this range is a passthrough
                new_seeds.push(range);
            } else {
                // push the mapped ranges
                new_seeds.extend(mapped);
            }
        }

        new_seeds
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn get_mapped_seeds(&self, seed_ranges: Vec<Range<u64>>) -> u64 {
        let mapped = self.maps.iter().fold(seed_ranges, |acc, m| m.process(acc));
        mapped.iter().map(|x| x.start).min().unwrap()
    }
}

fn problem1(input: &Input) -> u64 {
    let ranges = input.seeds.iter().map(|&start| start..start + 1).collect();
    input.get_mapped_seeds(ranges)
}

fn problem2(input: &Input) -> u64 {
    let ranges = input
        .seeds
        .chunks(2)
        .map(|c| {
            let start = c[0];
            let length = c[1];
            start..start + length
        })
        .collect();

    input.get_mapped_seeds(ranges)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 35)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 46)
    }
}
