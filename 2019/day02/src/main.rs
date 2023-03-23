use advent_2019_02::Intcode;
use common::nom::usize;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<usize>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(tag(","), usize)(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let mut program = Intcode::new(input);
    program.set_inputs(12, 2);
    program.execute()
}

fn problem2(input: &Input) -> usize {
    let expected = 19_690_720;
    let program = Intcode::new(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = program.clone();
            program.set_inputs(noun, verb);
            if program.execute() == expected {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, Intcode};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let mut program = Intcode::new(&input);
        let result = program.execute();
        assert_eq!(result, 3500);
    }
}
