use common::{answer, nom::single_digit, read_input};
use nom::{multi::many1, IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(single_digit).parse(input);

    result.unwrap().1
}

fn pattern(idx: usize, len: usize) -> Vec<i32> {
    let pattern = [
        [0].repeat(idx + 1),
        [1].repeat(idx + 1),
        [0].repeat(idx + 1),
        [-1].repeat(idx + 1),
    ];
    pattern
        .iter()
        .flatten()
        .cycle()
        .skip(1)
        .take(len)
        .copied()
        .collect()
}

fn fft(input: &[u32]) -> Vec<u32> {
    (0..input.len())
        .map(|idx| {
            let result: i32 = pattern(idx, input.len())
                .into_iter()
                .zip(input)
                .map(|(x, y)| x * (*y as i32))
                .sum();

            (result % 10).unsigned_abs()
        })
        .collect()
}

fn to_number(value: &[u32]) -> u32 {
    value.iter().fold(0, |acc, x| (acc * 10) + x)
}

fn problem1(input: &Input) -> u32 {
    let mut value = input.clone();
    for _n in 0..100 {
        value = fft(&value);
    }

    to_number(&value[0..8])
}

fn problem2(input: &Input) -> u32 {
    let offset = to_number(&input[0..7]) as usize;
    let mut value: Vec<u32> = input.repeat(10_000).iter().skip(offset).copied().collect();

    for _n in 0..100 {
        value.iter_mut().rev().fold(0, |acc, x| {
            *x = (acc + *x) % 10;
            *x
        });
    }

    to_number(&value[0..8])
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let cases = [
            ("80871224585914546619083218645595", 24176176),
            ("19617804207202209144916044189917", 73745418),
            ("69317163492948606335995924319873", 52432133),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem1(&input);
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn second() {
        let cases = [
            ("03036732577212944063491565474664", 84462026),
            ("02935109699940807407585447034323", 78725270),
            ("03081770884921959731165446850517", 53553731),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem2(&input);
            assert_eq!(result, expected)
        }
    }
}
