use common::{answer, read_input};
use intcode::Intcode;
use itertools::Itertools;

fn main() {
    let input = read_input!();
    let input = Intcode::parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Intcode;
fn execute_amplifier(program: &Intcode, phase: i64, input: i64) -> i64 {
    let mut p = program.clone();
    p.input = vec![input, phase];
    p.execute();
    p.get_last_output()
}

fn execute_phase(input: &Intcode, phase: &[i64]) -> i64 {
    let a = execute_amplifier(input, phase[0], 0);
    let b = execute_amplifier(input, phase[1], a);
    let c = execute_amplifier(input, phase[2], b);
    let d = execute_amplifier(input, phase[3], c);

    execute_amplifier(input, phase[4], d)
}

fn execute_phase_feedback(program: &Intcode, phase: &[i64]) -> i64 {
    let mut programs: Vec<Intcode> = phase
        .iter()
        .map(|phase| {
            let mut p = program.clone();
            p.input.push(*phase);
            // start the program so we don't have to deal with program a having both phase and 0 in the input
            p.execute();
            p
        })
        .collect();

    let mut current = 0;
    let mut last_output = 0;
    loop {
        if let Some(program) = programs.get_mut(current) {
            program.input.push(last_output);
            let result = program.execute();
            last_output = program.get_last_output();

            // if the last program has halted, bail with the result
            if result == intcode::ExecutionResult::Halted && current == 4 {
                return last_output;
            }

            // move to the next program
            current = (current + 1) % 5;
        }
    }
}

fn problem1(input: &Input) -> i64 {
    [0, 1, 2, 3, 4]
        .into_iter()
        .permutations(5)
        .map(|phase| execute_phase(input, &phase))
        .max()
        .unwrap()
}

fn problem2(input: &Input) -> i64 {
    [5, 6, 7, 8, 9]
        .into_iter()
        .permutations(5)
        .map(|phase| execute_phase_feedback(input, &phase))
        .max()
        .unwrap()
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
        assert_eq!(result, 43210)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 139629729)
    }
}
