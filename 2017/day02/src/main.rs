use nom::{
    character::complete::{newline, space1, u32},
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

type Input = Vec<Vec<u32>>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list1(newline, separated_list1(space1, u32))(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    input
        .iter()
        .map(|row| {
            let max = *row.iter().max().unwrap();
            let min = *row.iter().min().unwrap();

            max.abs_diff(min)
        })
        .sum()
}

fn problem2(input: &Input) -> u32 {
    input
        .iter()
        .map(|row| {
            row.iter()
                .find_map(|c1| {
                    row.iter()
                        .find_map(|c2| (c1 != c2 && c1 % c2 == 0).then(|| c1 / c2))
                })
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test1.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 18)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 9)
    }
}
