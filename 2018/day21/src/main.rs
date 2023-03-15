use advent_2018_19::ElfCode;

fn main() {
    let input = include_str!("../input.txt");
    let input = ElfCode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = ElfCode;

fn problem1(input: &Input) -> usize {
    let registers = execute(input, 0);
    registers[0]
}

fn execute(elfcode: &Input, starting: usize) -> Vec<usize> {
    let mut registers = vec![starting, 0, 0, 0, 0, 0];
    let mut ip = 0;

    while let Some(instruction) = elfcode.program.get(ip) {
        registers[elfcode.bound] = ip;
        registers = instruction.execute(&registers);
        ip = registers[elfcode.bound] + 1;
    }

    registers
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use advent_2018_19::ElfCode;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = ElfCode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = ElfCode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
