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
    let registers = execute(input, 0, false);
    registers[0]
}

fn execute(elfcode: &Input, starting: usize, terminate_early: bool) -> Vec<usize> {
    let mut registers = vec![starting, 0, 0, 0, 0, 0];
    let mut ip = 0;

    while let Some(instruction) = elfcode.program.get(ip) {
        // break as soon as the first register switches to zero, we've got our number
        if terminate_early && registers[0] != starting {
            break;
        }
        registers[elfcode.bound] = ip;
        registers = instruction.execute(&registers);
        ip = registers[elfcode.bound] + 1;
    }

    registers
}

fn problem2(input: &Input) -> usize {
    let registers = execute(input, 1, true);
    // our particular input happens to be in the fourth register
    let x = registers[3];
    // find the sum of all the divisors of the number (this is what the assembly is doing)
    (1..=x).filter(|n| x % n == 0).sum()
}

#[cfg(test)]
mod test {
    use advent_2018_19::ElfCode;

    use crate::problem1;
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = ElfCode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 6)
    }
}
