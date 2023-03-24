use intcode::Intcode;

fn main() {
    let input = include_str!("../input.txt");
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
}

type Input = Intcode;

fn problem1(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::problem1;
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }
}
