use nom::{
    bytes::complete::tag,
    character::complete::{anychar, none_of},
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(tag(","), many1(none_of(",\n")))(input);

    result.unwrap().1
}

fn hash(v: &[char]) -> u8 {
    v.iter()
        .fold(0, |acc, &c| acc.wrapping_add(c as u8).wrapping_mul(17))
}

#[test]
fn test_hash() {
    assert_eq!(hash(&['H', 'A', 'S', 'H']), 52);
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|x| hash(x) as u32).sum()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1320)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
