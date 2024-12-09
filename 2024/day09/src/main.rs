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

type Input = (VecDeque<File>, VecDeque<FreeSpace>);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(many1(map(single_digit, |x| x as u64)), |input| {
        let mut at_index = 0;
        let mut id = 0;
        let mut files: VecDeque<File> = VecDeque::new();
        let mut free: VecDeque<FreeSpace> = VecDeque::new();
        for (i, &size) in input.iter().enumerate() {
            if i % 2 == 0 {
                files.push_back(File { id, size, at_index });
                id += 1;
            } else {
                free.push_back(FreeSpace { size, at_index });
            }

            at_index += size;
        }

        (files, free)
    })(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
struct File {
    id: u64,
    size: u64,
    at_index: u64,
}

impl File {
    fn checksum(&self) -> u64 {
        (0..self.size).map(|i| self.id * (self.at_index + i)).sum()
    }
}

#[derive(Clone, Copy, Debug)]
struct FreeSpace {
    size: u64,
    at_index: u64,
}

fn defrag(input: &Input, mut predicate: impl FnMut(&FreeSpace, &File) -> bool) -> u64 {
    let (mut files, mut free) = input.clone();

    let mut checksum = 0;
    while let Some(mut file) = files.pop_back() {
        // find the first free space that is bigger and to the left of the file
        let f = match free
            .iter_mut()
            .find(|x| predicate(x, &file) && x.at_index <= file.at_index)
        {
            Some(first_available) => {
                // update the file to be there, decrease the free space
                let chunks_to_move = file.size.min(first_available.size);
                let chunk = File {
                    id: file.id,
                    size: chunks_to_move,
                    at_index: first_available.at_index,
                };

                first_available.at_index += chunks_to_move;
                first_available.size -= chunks_to_move;
                file.size -= chunks_to_move;

                if file.size > 0 {
                    files.push_back(file);
                }

                chunk
            }
            None => file,
        };

        checksum += f.checksum();
    }

    checksum
}

fn problem1(input: &Input) -> u64 {
    defrag(input, |free, _file| free.size > 0)
}

fn problem2(input: &Input) -> u64 {
    defrag(input, |free, file| free.size >= file.size)
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
