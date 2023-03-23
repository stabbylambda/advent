use nom::{
    character::complete::{newline, u32},
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

type Input = Vec<u32>;

fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn recursive_fuel(mut mass: u32) -> u32 {
    let mut total = 0;

    loop {
        mass = fuel(mass);
        total += mass;

        if mass == 0 {
            break total;
        }
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, u32)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|x| fuel(*x)).sum()
}

fn problem2(input: &Input) -> u32 {
    input.iter().map(|x| recursive_fuel(*x)).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, recursive_fuel};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        let expected = 2 + 2 + 654 + 33583;
        assert_eq!(result, expected)
    }

    #[test]
    fn second() {
        let cases = [(14, 2), (1969, 966), (100756, 50346)];
        for (input, expected) in cases {
            let result = recursive_fuel(input);
            assert_eq!(result, expected);
        }
    }
}
