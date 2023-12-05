use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Almanac;

fn parse(input: &str) -> Input {
    let numbers = |s| separated_list1(tag(" "), u64)(s);
    let almanac_range = map(separated_list1(tag(" "), u64), |v| AlmanacRange {
        destination_start: v[0],
        source_start: v[1],
        length: v[2],
    });
    let almanac_maps = separated_list1(
        tag("\n\n"),
        map(
            tuple((
                terminated(take_until(" "), tag(" map:\n")),
                separated_list1(newline, almanac_range),
            )),
            |(_name, ranges)| AlmanacMap { ranges },
        ),
    );

    let result: IResult<&str, Input> = map(
        separated_pair(preceded(tag("seeds: "), numbers), tag("\n\n"), almanac_maps),
        |(seeds, maps)| Almanac { seeds, maps },
    )(input);

    result.unwrap().1
}

#[derive(Debug)]
struct AlmanacRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacRange {
    fn translate(&self, n: u64) -> Option<u64> {
        let range = self.source_start..self.source_start + self.length;

        if range.contains(&n) {
            // map the value in the range
            Some(self.destination_start + (n - self.source_start))
        } else {
            // this range doesn't map the value
            None
        }
    }
}

#[derive(Debug)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn translate(&self, n: u64) -> u64 {
        // do any of the ranges map this number?
        if let Some(mapped) = self.ranges.iter().find_map(|x| x.translate(n)) {
            mapped
        } else {
            // if not, return the number
            n
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn translate_all(&self) -> Vec<u64> {
        self.seeds.iter().map(|x| self.translate(*x)).collect()
    }
    fn translate(&self, n: u64) -> u64 {
        self.maps
            .iter()
            .fold(n, |current, map| map.translate(current))
    }
}

fn problem1(input: &Input) -> u64 {
    *input.translate_all().iter().min().unwrap()
}

fn problem2(_input: &Input) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2, Almanac, AlmanacMap, AlmanacRange};

    #[test]
    fn range_translate() {
        let r = AlmanacRange {
            destination_start: 50,
            source_start: 98,
            length: 2,
        };

        assert_eq!(r.translate(1), None);
        assert_eq!(r.translate(98), Some(50));
        assert_eq!(r.translate(99), Some(51));
        assert_eq!(r.translate(100), None);
    }

    #[test]
    fn map_translate() {
        let r1 = AlmanacRange {
            destination_start: 50,
            source_start: 98,
            length: 2,
        };
        let r2 = AlmanacRange {
            destination_start: 52,
            source_start: 50,
            length: 48,
        };

        let map = AlmanacMap {
            ranges: vec![r1, r2],
        };

        assert_eq!(map.translate(0), 0);
        assert_eq!(map.translate(1), 1);
        assert_eq!(map.translate(48), 48);
        assert_eq!(map.translate(49), 49);
        assert_eq!(map.translate(50), 52);
        assert_eq!(map.translate(51), 53);
        assert_eq!(map.translate(96), 98);
        assert_eq!(map.translate(97), 99);
        assert_eq!(map.translate(98), 50);
        assert_eq!(map.translate(99), 51);
    }

    #[test]
    fn almanac_translate() {
        let input = include_str!("../test.txt");
        let a = parse(input);

        assert_eq!(a.translate(79), 82);
        assert_eq!(a.translate(14), 43);
        assert_eq!(a.translate(55), 86);
        assert_eq!(a.translate(13), 35);
    }

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
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
