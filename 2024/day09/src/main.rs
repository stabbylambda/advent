use std::{collections::VecDeque, time::Instant};

use common::nom::single_digit;
use nom::{combinator::map, multi::many1, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = Vec<u64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(map(single_digit, |x| x as u64))(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
struct ChunkedFile {
    id: u64,
    remaining: u64,
}

fn problem1(input: &Input) -> u64 {
    let mut files: VecDeque<ChunkedFile> = input
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(idx, x)| ChunkedFile {
            id: idx as u64,
            remaining: *x,
        })
        .collect();

    let mut free: VecDeque<u64> = input.iter().skip(1).step_by(2).copied().collect();

    let mut reading_file = true;
    let mut i = 0;
    let mut checksum = 0;
    while !files.is_empty() {
        if reading_file {
            // read a whole file in and figure out its checksum based on where it is
            if let Some(whole_file) = files.pop_front() {
                for _ in 0..whole_file.remaining {
                    checksum += whole_file.id * i;
                    i += 1;
                }

                reading_file = false;
            }
        } else {
            // start taking chunks from files on the back of the list
            if let Some(free_space) = free.pop_front() {
                for _ in 0..free_space {
                    if let Some(mut chunk_file) = files.pop_back() {
                        checksum += chunk_file.id * i;
                        chunk_file.remaining -= 1;

                        if chunk_file.remaining > 0 {
                            files.push_back(chunk_file);
                        }
                        i += 1;
                    }
                }

                reading_file = true;
            }
        }
    }

    checksum
}

#[derive(Clone, Copy, Debug)]
struct ContiguousFile {
    id: u64,
    size: u64,
    at_index: u64,
}

#[derive(Clone, Copy, Debug)]
struct FreeSpace {
    size: u64,
    at_index: u64,
}

fn problem2(input: &Input) -> u64 {
    let mut idx = 0;
    let mut id = 0;
    let mut files: Vec<ContiguousFile> = Vec::new();
    let mut free: Vec<FreeSpace> = Vec::new();
    for (i, x) in input.iter().enumerate() {
        // files are odd
        if i % 2 == 0 {
            files.push(ContiguousFile {
                id,
                size: *x,
                at_index: idx,
            });
            id += 1;
            idx += *x;
        } else {
            // free space is even, just increment the index
            free.push(FreeSpace {
                size: *x,
                at_index: idx,
            });
            idx += x;
        }
    }

    for file in files.iter_mut().rev() {
        // find the first free space that is bigger and to the left of the file
        if let Some(first_available) = free
            .iter_mut()
            .find(|x| x.size >= file.size && x.at_index <= file.at_index)
        {
            // update the file to be there, decrease the free space
            file.at_index = first_available.at_index;
            first_available.at_index += file.size;
            first_available.size -= file.size;
        }
    }

    files
        .iter()
        .map(|f| (0..f.size).map(|i| f.id * (f.at_index + i)).sum::<u64>())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1928)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 2858)
    }
}
