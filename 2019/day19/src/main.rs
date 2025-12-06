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

struct Robot<'a>(&'a Intcode);

impl<'a> Robot<'a> {
    fn pulled(&self, x: i64, y: i64) -> bool {
        let mut program = self.0.clone();
        program.input.push(y);
        program.input.push(x);

        program.execute();

        program.get_last_output() == 1
    }
}

fn problem1(input: &Input) -> i64 {
    let robot = Robot(input);
    let area = 50;
    (0..area)
        .map(|y| (0..area).map(|x| robot.pulled(x, y) as i64).sum::<i64>())
        .sum()
}

fn problem2(input: &Input) -> i64 {
    let robot = Robot(input);
    let mut x = 0;
    let mut y = 0;
    while !robot.pulled(x + 99, y) {
        y += 1;

        while !robot.pulled(x, y + 99) {
            x += 1;
        }
    }

    x * 10_000 + y
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 201)
    }

    #[test]
    fn second() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 6610984)
    }
}
