use common::{answer, digits, nom::usize, read_input, to_number};
use nom::{combinator::map, IResult, Parser};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<usize>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(map(usize, digits), |x| {
        x.iter().map(|x| *x as usize).collect()
    }).parse(input);

    result.unwrap().1
}

fn successors(input: &[usize]) -> Vec<usize> {
    let mut cups = vec![0; input.len() + 1];
    for x in input.windows(2) {
        cups[x[0]] = x[1];
    }
    cups[*input.last().unwrap()] = *input.first().unwrap();

    cups
}

fn game(cups: &[usize], mut cup: usize, rounds: u32) -> Vec<usize> {
    let mut cups = cups.to_vec();
    let max = cups.len();
    let wrapping_sub = |x: usize| match x.checked_sub(1) {
        Some(x) if x > 0 => x,
        _ => max - 1,
    };

    for _round in 0..rounds {
        let a = cups[cup];
        let b = cups[a];
        let c = cups[b];

        let mut dest = wrapping_sub(cup);
        while dest == a || dest == b || dest == c {
            dest = wrapping_sub(dest);
        }

        cups[cup] = cups[c];
        let temp = cups[dest];
        cups[dest] = a;
        cups[c] = temp;
        cup = cups[cup];
    }

    cups
}

fn problem1(input: &Input) -> u32 {
    let cup = *input.first().unwrap();

    let cups = successors(input);
    let cups = game(&cups, cup, 100);

    let mut answer = vec![];
    let mut i = 1;
    loop {
        match cups[i] {
            1 => break,
            cup => {
                answer.push(cup as u32);
                i = cup;
            }
        }
    }

    to_number(&answer)
}

fn problem2(input: &Input) -> usize {
    let cup = *input.first().unwrap();

    let mut cups = input.to_vec();
    cups.extend(10..=1_000_000);
    let cups = successors(&cups);
    let cups = game(&cups, cup, 10_000_000);

    cups[1] * cups[cups[1]]
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 67384529)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 149_245_887_792)
    }
}
