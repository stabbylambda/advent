use core::panic;
use std::{fmt::Debug, iter::Sum, ops::Add};

use nom::{
    branch::alt,
    character::complete::{char, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
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

#[derive(PartialEq, Eq, Clone)]
enum Snailfish {
    Literal(u32),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

enum ExplodeResult {
    Exploded(Option<u32>, Option<u32>),
    None,
}

impl Snailfish {
    fn new(input: &str) -> Snailfish {
        Snailfish::parse(input).unwrap().1
    }

    fn explode(&mut self) -> bool {
        match self._explode(0) {
            ExplodeResult::Exploded(_, _) => true,
            ExplodeResult::None => false,
        }
    }

    fn _explode(&mut self, depth: u32) -> ExplodeResult {
        match self {
            Snailfish::Pair(l, r) if depth >= 4 => {
                // this pair is too deep, so we explode
                let Snailfish::Literal(l) = l.as_ref() else { panic!("Somehow we got past depth 4") };
                let Snailfish::Literal(r) = r.as_ref() else { panic!("Somehow we got past depth 4") };

                ExplodeResult::Exploded(Some(*l), Some(*r))
            }
            Snailfish::Pair(l, r) => {
                // try the left side first
                if let ExplodeResult::Exploded(ll, lr) = l._explode(depth + 1) {
                    if ll.is_some() && lr.is_some() {
                        //replace the pair with a zero
                        *l = Box::new(Snailfish::Literal(0))
                    }

                    r.add_left(lr);

                    ExplodeResult::Exploded(ll, None)
                } else if let ExplodeResult::Exploded(rl, rr) = r._explode(depth + 1) {
                    if rl.is_some() && rr.is_some() {
                        //replace the pair with a zero
                        *r = Box::new(Snailfish::Literal(0))
                    }

                    l.add_right(rl);

                    ExplodeResult::Exploded(None, rr)
                } else {
                    ExplodeResult::None
                }
            }
            Snailfish::Literal(_) => ExplodeResult::None,
        }
    }

    fn add_right(&mut self, x: Option<u32>) {
        match (x, self) {
            (Some(x), Snailfish::Literal(v)) => *v += x,
            (Some(_), Snailfish::Pair(_, r)) => r.add_right(x),
            _ => {}
        }
    }

    fn add_left(&mut self, x: Option<u32>) {
        match (x, self) {
            (Some(x), Snailfish::Literal(v)) => *v += x,
            (Some(_), Snailfish::Pair(l, _)) => l.add_left(x),
            _ => {}
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Literal(x) if *x >= 10 => {
                *self = Snailfish::Pair(
                    Box::new(Snailfish::Literal(*x / 2)),
                    Box::new(Snailfish::Literal(*x / 2 + *x % 2)),
                );
                true
            }
            Snailfish::Pair(l, r) => l.split() || r.split(),
            _ => false,
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                delimited(
                    char('['),
                    separated_pair(Self::parse, char(','), Self::parse),
                    char(']'),
                ),
                |(l, r)| Snailfish::Pair(Box::new(l), Box::new(r)),
            ),
            map(nom_u32, Snailfish::Literal),
        ))(input)
    }

    fn magnitude(&self) -> u32 {
        match self {
            Snailfish::Literal(x) => *x,
            Snailfish::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn reduce(&mut self) {
        loop {
            // first try exploding
            if self.explode() {
                continue;
            }

            // if that didn't work, try splitting
            if self.split() {
                continue;
            }

            // if neither work, we're done
            break;
        }
    }
}

impl Debug for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => write!(f, "{arg0}"),
            Self::Pair(arg0, arg1) => write!(f, "[{arg0:?},{arg1:?}]"),
        }
    }
}

impl Add for Snailfish {
    type Output = Snailfish;

    fn add(self, rhs: Self) -> Self::Output {
        let mut s = Snailfish::Pair(Box::new(self), Box::new(rhs));
        s.reduce();
        s
    }
}

impl Sum for Snailfish {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap()
    }
}

type Input = Vec<Snailfish>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, Snailfish::parse)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    let s: Snailfish = input.iter().cloned().sum();
    s.magnitude()
}

fn problem2(input: &Input) -> u32 {
    let mut max = 0;
    for x in input.iter() {
        for y in input.iter() {
            let s1 = (x.clone() + y.clone()).magnitude();
            let s2 = (y.clone() + x.clone()).magnitude();

            max = s1.max(s2).max(max);
        }
    }

    max
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2, Snailfish};

    fn snailfish(input: &str) -> Snailfish {
        Snailfish::parse(input).unwrap().1
    }

    #[test]
    fn split() {
        let cases = [
            ("1", "1"),
            ("[1,1]", "[1,1]"),
            ("10", "[5,5]"),
            ("11", "[5,6]"),
            ("12", "[6,6]"),
        ];
        for (input, expected) in cases {
            let mut s = Snailfish::new(input);
            s.split();
            assert_eq!(s, Snailfish::new(expected));
        }
    }

    #[test]
    fn magnitude() {
        let cases = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (input, expected) in cases {
            let s = Snailfish::new(input);
            assert_eq!(s.magnitude(), expected);
        }
    }

    #[test]
    fn explode() {
        let cases = [
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        // examples from the problem
        for (input, expected) in cases {
            let mut s = Snailfish::new(input);
            s.explode();
            assert_eq!(s, Snailfish::new(expected));
        }
    }

    #[test]
    fn reduce() {
        let mut input = Snailfish::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        input.reduce();
        assert_eq!(input, Snailfish::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn add() {
        let result = Snailfish::new("[[[[4,3],4],4],[7,[[8,4],9]]]") + Snailfish::new("[1,1]");
        assert_eq!(result, Snailfish::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn slightly_larger() {
        let s = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let nums = parse(&s);
        let result: Snailfish = nums.iter().cloned().sum();
        assert_eq!(
            result,
            Snailfish::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 4140)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 3993)
    }
}
