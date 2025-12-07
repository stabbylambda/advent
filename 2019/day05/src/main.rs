use common::{answer, read_input};
use intcode::Intcode;

fn main() {
    let input = read_input!();
    let input = Intcode::parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Intcode;

fn problem1(input: &Input) -> i64 {
    let mut program = input.clone();
    program.execute_simple(1)
}

fn problem2(input: &Input) -> i64 {
    let mut program = input.clone();
    program.execute_simple(5)
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::{problem1, problem2};
    #[test]
    #[ignore = "input files aren't available in CI"]
    fn first() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 13210611)
    }

    #[test]
    #[ignore = "input files aren't available in CI"]
    fn second() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 584126)
    }
}
