use intcode::Intcode;

fn main() {
    let input = common::read_input!();
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Intcode;

fn problem1(input: &Input) -> i64 {
    let mut program = input.clone();
    program.set_noun_verb(12, 2);
    program.execute();
    program.get_location0()
}

fn problem2(input: &Input) -> i64 {
    let expected = 19_690_720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input.clone();
            program.set_noun_verb(noun, verb);
            program.execute();
            if program.get_location0() == expected {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::Intcode;
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let mut program = Intcode::parse(input);
        program.execute();
        assert_eq!(program.get_location0(), 3500);
    }
}
