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

struct Springdroid {
    program: Intcode,
}

impl Springdroid {
    fn execute_program(&self, input: &[&str]) -> Result<i64, String> {
        let mut program = self.program.clone();
        program.input.push('\n' as i64);
        for c in input.join("\n").chars().rev() {
            program.input.push(c as i64);
        }

        program.execute();
        match program.output.last() {
            Some(&x) if x > 255 => Ok(x),
            _ => Err(program.output.iter().map(|x| (*x as u8) as char).collect()),
        }
    }
}

fn problem1(input: &Input) -> i64 {
    let droid = Springdroid {
        program: input.clone(),
    };
    let result = droid.execute_program(&[
        "NOT B J", // is there a hole at B
        "NOT C T", // is there a hole at C
        "OR T J",  // is one of B or C a hole
        "AND D J", // is one of B or C a hole and D is ground (the early jump scenario)
        "NOT A T", // are we standing in front of a hole and we'll die if we fall (the last minute scenario)
        "OR T J",  // jump if early jump or last minute
        "WALK",
    ]);
    match result {
        Ok(damage) => damage,
        Err(death) => {
            println!("{death}");
            0
        }
    }
}

fn problem2(input: &Input) -> i64 {
    let droid = Springdroid {
        program: input.clone(),
    };
    let result = droid.execute_program(&[
        "NOT B J", // is there a hole at B
        "NOT C T", // is there a hole at C
        "OR T J",  // is one of B or C a hole
        "AND D J", // is one of B or C a hole and D is ground
        "AND H J", // is one of B or C a hole and D and H are both ground? (the early jump scenario under RUN)
        "NOT A T", // are we standing in front of a hole and we'll die if we fall (the last minute scenario)
        "OR T J",  // jump if early jump or last minute
        "RUN",
    ]);
    match result {
        Ok(damage) => damage,
        Err(death) => {
            println!("{death}");
            0
        }
    }
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
        assert_eq!(result, 19348840)
    }

    #[test]
    fn second() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1141857182)
    }
}
