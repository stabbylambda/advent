use itertools::Itertools;

use common::get_raw_input;
use nom::{
    character::complete::{alpha1, anychar, char, newline, u32 as nom_u32},
    combinator::map,
    multi::{count, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Room>;

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: Vec<char>,
}

impl Room {
    fn is_valid(&self) -> bool {
        let by_frequency = self
            .name
            .chars()
            .filter(|c| c.is_alphabetic()) // drop the spaces
            .counts_by(|x| x)
            .into_iter()
            // sort by frequency first, then lexicographically
            .sorted_by(|(a_char, a_count), (b_char, b_count)| {
                b_count.cmp(a_count).then(a_char.cmp(b_char))
            })
            .map(|x| x.0)
            .take(5)
            .collect_vec();

        by_frequency == self.checksum
    }

    fn decrypt(&self) -> String {
        let a = 'a' as u8;
        let space = ' ' as u8;
        let sector_id = (self.sector_id % 26) as u8;

        self.name
            .bytes()
            .map(|b| {
                if b == space {
                    space as char
                } else {
                    // addition under mod 26 lets us wrap around the alphabet, but we need to treat 'a' as zero
                    (((b - a + sector_id) % 26) + a) as char
                }
            })
            .join("")
    }
}

fn checksum(input: &str) -> IResult<&str, Vec<char>> {
    delimited(char('['), count(anychar, 5), char(']'))(input)
}

fn name(input: &str) -> IResult<&str, String> {
    map(separated_list1(char('-'), alpha1), |x| x.join(" "))(input)
}

fn parse(input: &str) -> Input {
    let sector_id = preceded(char('-'), nom_u32);

    let room = map(
        tuple((name, sector_id, checksum)),
        |(name, sector_id, checksum)| Room {
            name,
            sector_id,
            checksum,
        },
    );
    let result: IResult<&str, Input> = separated_list1(newline, room)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(|x| x.is_valid().then_some(x.sector_id))
        .sum()
}

fn problem2(input: &Input) -> u32 {
    input
        .iter()
        .filter(|x| x.decrypt() == "northpole object storage")
        .map(|x| x.sector_id)
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 1514)
    }
}
