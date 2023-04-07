use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
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

type Input = Vec<Vec<String>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        tag("\n\n"),
        separated_list1(newline, map(alpha1, |x: &str| x.to_string())),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input
        .iter()
        .map(|answers| {
            ('a'..='z')
                .filter(|c| answers.iter().any(|a| a.contains(*c)))
                .count()
        })
        .sum()
}

fn problem2(input: &Input) -> usize {
    input
        .iter()
        .map(|answers| {
            ('a'..='z')
                .filter(|c| answers.iter().all(|a| a.contains(*c)))
                .count()
        })
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
        assert_eq!(result, 11)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 6)
    }
}
