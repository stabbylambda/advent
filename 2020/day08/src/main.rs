use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Instruction>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                alt((
                    map(tag("nop"), |_| InstructionType::Nop),
                    map(tag("acc"), |_| InstructionType::Acc),
                    map(tag("jmp"), |_| InstructionType::Jmp),
                )),
                tag(" "),
                i32,
            ),
            |(name, offset)| Instruction {
                name,
                offset,
                executed: 0,
            },
        ),
    ).parse(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    name: InstructionType,
    offset: i32,
    executed: u32,
}

#[derive(Clone, Copy, Debug)]
enum InstructionType {
    Nop,
    Acc,
    Jmp,
}

enum ExecutionResult {
    Halted,
    InfiniteLoop,
}

fn execute(input: &Input) -> (ExecutionResult, i32) {
    let mut boot_code = input.clone();
    let mut acc = 0;
    let mut pc = 0;

    while let Some(instruction) = boot_code.get_mut(pc) {
        if instruction.executed == 1 {
            return (ExecutionResult::InfiniteLoop, acc);
        }
        match instruction.name {
            InstructionType::Nop => {
                pc += 1;
            }
            InstructionType::Acc => {
                acc += instruction.offset;
                pc += 1;
            }
            InstructionType::Jmp => {
                pc = pc.saturating_add_signed(instruction.offset as isize);
            }
        }

        instruction.executed += 1;
    }

    (ExecutionResult::Halted, acc)
}

fn problem1(input: &Input) -> i32 {
    execute(input).1
}

fn problem2(input: &Input) -> i32 {
    for idx in 0..input.len() {
        let instruction = input[idx];
        let new_program = match instruction.name {
            // don't bother executing the program on an acc
            InstructionType::Acc => {
                continue;
            }
            // if it's a nop or jump, swap the instruction
            InstructionType::Nop => {
                let mut new_program = input.clone();
                new_program[idx].name = InstructionType::Jmp;
                new_program
            }
            InstructionType::Jmp => {
                let mut new_program = input.clone();
                new_program[idx].name = InstructionType::Nop;
                new_program
            }
        };

        match execute(&new_program) {
            (ExecutionResult::Halted, acc) => return acc,
            _ => continue,
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 5)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 8)
    }
}
