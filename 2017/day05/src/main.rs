use nom::{
    character::complete::{i32, newline},
    multi::separated_list1,
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

type Input = Vec<i32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(newline, i32)(input);

    result.unwrap().1
}

fn simulate(input: &Input, f: impl Fn(i32) -> i32) -> u32 {
    let mut instructions = input.clone();
    let mut count = 0;
    let mut current: i32 = 0;

    while let Some(instruction) = instructions.get_mut(current as usize) {
        // move the pc
        current += *instruction;

        // if we jumped off the back, casting to usize will panic, so break before that
        if current < 0 {
            break;
        }

        // increment the instruction
        *instruction += f(*instruction);

        count += 1;
    }

    count
}

fn problem1(input: &Input) -> u32 {
    simulate(input, |_| 1)
}

fn problem2(input: &Input) -> u32 {
    simulate(input, |i| if i >= 3 { -1 } else { 1 })
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
        assert_eq!(result, 10)
    }
}
