use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i128, newline},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Technique>;

enum Technique {
    DealNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(tag("deal into new stack"), |_| Technique::DealNewStack),
            map(
                preceded(tag("deal with increment "), i128),
                Technique::DealWithIncrement,
            ),
            map(preceded(tag("cut "), i128), Technique::Cut),
        )),
    ).parse(input);

    result.unwrap().1
}

fn shuffle(input: &Input, deck: &[usize]) -> Vec<usize> {
    let mut deck = deck.to_vec();
    let card_count = deck.len();

    for technique in input {
        match technique {
            Technique::DealNewStack => deck.reverse(),
            Technique::Cut(x) => {
                let top = if x.is_positive() {
                    *x as usize
                } else {
                    card_count - (x.unsigned_abs() as usize)
                };

                deck = deck
                    .iter()
                    .cycle()
                    .skip(top)
                    .take(card_count)
                    .copied()
                    .collect();
            }
            Technique::DealWithIncrement(x) => {
                let mut current = 0;
                let mut hand: VecDeque<usize> = deck.iter().copied().collect();

                while let Some(card) = hand.pop_front() {
                    deck[current] = card;
                    current = (current + (*x as usize)) % card_count;
                }
            }
        }
    }

    deck
}

fn problem1(input: &Input) -> usize {
    let deck: Vec<usize> = (0..10_007).collect();
    let result = shuffle(input, &deck);

    result.iter().position(|x| *x == 2019).unwrap()
}

fn problem2(input: &Input) -> i128 {
    // I super cheated on this. No way I was ever getting this myself
    const CARDS: i128 = 119_315_717_514_047;
    const ITERATIONS: i128 = 101_741_582_076_661;

    let mut a = 1;
    let mut b = 0;

    for op in input.iter().rev() {
        match op {
            &Technique::Cut(n) => {
                b += if n < 0 { n + CARDS } else { n };
            }
            &Technique::DealWithIncrement(n) => {
                let inv = modinv(n, CARDS);
                a *= inv % CARDS;
                b *= inv % CARDS;
            }
            Technique::DealNewStack => {
                b = -(b + 1);
                a *= -1;
            }
        }

        a %= CARDS;
        b %= CARDS;

        if a < 0 {
            a += CARDS;
        }

        if b < 0 {
            b += CARDS;
        }
    }

    let x = modp(a, ITERATIONS, CARDS);
    let i1 = x * 2020 % CARDS;
    let i2 = (x + CARDS - 1) % CARDS;
    let i3 = b * i2 % CARDS;
    let i4 = modp(a - 1, CARDS - 2, CARDS);

    (i1 + i3 * i4) % CARDS
}

fn modinv(mut a: i128, mut base: i128) -> i128 {
    if base == 1 {
        return 0;
    }

    let orig = base;

    let mut x = 1;
    let mut y = 0;

    while a > 1 {
        let q = a / base;
        let tmp = base;
        base = a % base;
        a = tmp;
        let tmp = y;
        y = x - q * y;
        x = tmp;
    }

    if x < 0 {
        x + orig
    } else {
        x
    }
}

fn modp(b: i128, exp: i128, base: i128) -> i128 {
    let mut x = 1;
    let mut p = b % base;

    for i in 0..128 {
        if 1 & (exp >> i) == 1 {
            x = x * p % base;
        }

        p = p * p % base;
    }

    x
}

#[cfg(test)]
mod test {
    use crate::{parse, problem2, shuffle};
    #[test]
    fn small_deck() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let deck: Vec<usize> = (0..10).collect();
        let result = shuffle(&input, &deck);
        assert_eq!(result, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 117607927195067)
    }
}
