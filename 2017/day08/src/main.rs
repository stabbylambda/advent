use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{alpha1, i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use registers::{Register, Registers};
pub mod registers;

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input<'a> = Vec<Instruction<'a>>;

#[derive(Debug)]
enum Action {
    Increment(i32),
    Decrement(i32),
}

#[derive(Debug)]
enum ConditionType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug)]
struct Condition<'a> {
    register: Register<'a>,
    kind: ConditionType,
    value: i32,
}

impl<'a> Condition<'a> {
    fn evaluate(&self, registers: &mut Registers<'a>) -> bool {
        let r_value = registers.get(self.register);
        match self.kind {
            ConditionType::Equal => r_value == self.value,
            ConditionType::NotEqual => r_value != self.value,
            ConditionType::GreaterThan => r_value > self.value,
            ConditionType::GreaterThanOrEqual => r_value >= self.value,
            ConditionType::LessThan => r_value < self.value,
            ConditionType::LessThanOrEqual => r_value <= self.value,
        }
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    register: Register<'a>,
    action: Action,
    condition: Condition<'a>,
}

impl<'a> Instruction<'a> {
    fn execute(&self, registers: &mut Registers<'a>) {
        let cond_pass = self.condition.evaluate(registers);
        if cond_pass {
            match self.action {
                Action::Increment(x) => registers.inc(self.register, x),
                Action::Decrement(x) => registers.dec(self.register, x),
            }
        }
    }
}

fn parse(input: &str) -> Input<'_> {
    let action = alt((
        map(preceded(tag("inc "), i32), Action::Increment),
        map(preceded(tag("dec "), i32), Action::Decrement),
    ));

    let condition = map(
        (
            delimited(tag("if "), alpha1, tag(" ")),
            take_until1(" "),
            preceded(tag(" "), i32),
        ),
        |(register, cond, value)| {
            let kind = match cond {
                "==" => ConditionType::Equal,
                "!=" => ConditionType::NotEqual,
                ">" => ConditionType::GreaterThan,
                ">=" => ConditionType::GreaterThanOrEqual,
                "<" => ConditionType::LessThan,
                "<=" => ConditionType::LessThanOrEqual,
                _ => unreachable!(),
            };

            Condition {
                register,
                kind,
                value,
            }
        },
    );

    let instruction = map(
        (
            terminated(alpha1, tag(" ")),
            terminated(action, tag(" ")),
            condition,
        ),
        |(register, action, condition)| Instruction {
            register,
            action,
            condition,
        },
    );

    let result: IResult<&str, Input> = separated_list1(newline, instruction).parse(input);

    result.unwrap().1
}

fn problem(input: &Input) -> (i32, i32) {
    let mut registers = Registers::new();
    let mut max_all_time = 0;
    for i in input {
        i.execute(&mut registers);
        max_all_time = max_all_time.max(registers.max())
    }
    (registers.max(), max_all_time)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (result1, result2) = problem(&input);
        assert_eq!(result1, 1);
        assert_eq!(result2, 10);
    }
}
