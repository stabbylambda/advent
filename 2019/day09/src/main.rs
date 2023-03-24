use intcode::Intcode;

fn main() {
    let input = include_str!("../input.txt");
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Intcode;

fn problem1(input: &Input) -> i64 {
    let mut p = input.clone();
    p.execute_simple(1)
}

fn problem2(input: &Input) -> i64 {
    let mut p = input.clone();
    p.execute_simple(2)
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 1125899906842624)
    }
}
