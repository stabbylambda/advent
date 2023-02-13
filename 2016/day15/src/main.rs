use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Disc>;

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures
fn inverse(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(x, n);
    (g == 1).then_some((x % n + n) % n)
}

// and finally https://en.wikipedia.org/wiki/Chinese_remainder_theorem
// this only works because everything is pairwise coprime
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod: i64 = modulii.iter().product();

    let mut sum = 0;
    let pairs = residues.iter().zip(modulii);

    for (&residue, &modulus) in pairs {
        let p = prod / modulus;
        sum += residue * inverse(p, modulus)? * p
    }

    Some(sum % prod)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            tuple((
                preceded(tag("Disc #"), nom_i64),
                delimited(tag(" has "), nom_i64, tag(" positions;")),
                delimited(tag(" at time=0, it is at position "), nom_i64, tag(".")),
            )),
            |(_number, size, start)| Disc { size, start },
        ),
    )(input);

    result.unwrap().1
}
#[derive(Clone)]
struct Disc {
    size: i64,
    start: i64,
}

fn problem(input: &Input) -> i64 {
    let residues: Vec<i64> = input
        .iter()
        .enumerate()
        .map(|(seconds, x)| {
            // adjust for the falling time + modulus
            let position_at_t = x.start + (seconds as i64) + 1;
            x.size - (position_at_t % x.size)
        })
        .collect();

    let modulii: Vec<i64> = input.iter().map(|x| x.size).collect();

    chinese_remainder(&residues, &modulii).unwrap()
}

fn problem1(input: &Input) -> i64 {
    problem(input)
}

fn problem2(input: &Input) -> i64 {
    let mut input = input.clone();
    input.push(Disc { size: 11, start: 0 });

    problem(&input)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 5)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 85)
    }
}
