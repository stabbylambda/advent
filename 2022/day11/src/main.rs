use nom::branch::alt;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64 as nom_u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let mut monkeys = parse(input);

    let answer = problem1(&mut monkeys);
    println!("problem 1 answer: {answer}");

    let mut monkeys = parse(input);
    let answer = problem2(&mut monkeys);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Monkey>;
#[derive(Debug)]
struct Monkey {
    number: u64,
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

#[derive(Debug)]
enum OperationValue {
    Constant(u64),
    Old,
}
#[derive(Debug)]
enum Operation {
    Add(OperationValue),
    Mul(OperationValue),
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        separated_pair(
            alt((
                nom::character::complete::char('+'),
                nom::character::complete::char('*'),
            )),
            tag(" "),
            alt((
                map(nom_u64, OperationValue::Constant),
                map(tag("old"), |_| OperationValue::Old),
            )),
        ),
        |(op, value)| match op {
            '+' => Operation::Add(value),
            '*' => Operation::Mul(value),
            _x => panic!("couldn't parse operation {_x}"),
        },
    )(input)
}
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            delimited(tag("Monkey "), nom_u64, tag(":\n")),
            delimited(
                tag("  Starting items: "),
                separated_list1(tag(", "), nom_u64),
                newline,
            ),
            delimited(tag("  Operation: new = old "), parse_operation, newline),
            delimited(tag("  Test: divisible by "), nom_u64, newline),
            delimited(tag("    If true: throw to monkey "), nom_u64, newline),
            preceded(tag("    If false: throw to monkey "), nom_u64),
        )),
        |(number, items, operation, divisible_by, if_true, if_false)| {
            let items = items.to_vec();
            Monkey {
                number,
                items,
                operation,
                divisible_by,
                if_true: if_true as usize,
                if_false: if_false as usize,
                inspected: 0,
            }
        },
    )(input)
}
fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(tag("\n\n"), parse_monkey)(input);

    result.unwrap().1
}

type ThrowTo = (u64, usize);

impl Monkey {
    fn inspect_all(&mut self, worry_divisor: Option<u64>) -> Vec<ThrowTo> {
        // figure out where all the items are going
        let results: Vec<(u64, usize)> = self
            .items
            .iter()
            .map(|item| self.inspect(item, worry_divisor))
            .collect();

        self.inspected += results.len();

        // clear out this monkey's items
        self.items.clear();

        results
    }
    fn inspect(&self, item: &u64, worry_divisor: Option<u64>) -> ThrowTo {
        // do the operation
        let item = match &self.operation {
            Operation::Add(v) => {
                item + match v {
                    OperationValue::Constant(x) => x,
                    OperationValue::Old => item,
                }
            }
            Operation::Mul(v) => {
                item * match v {
                    OperationValue::Constant(x) => x,
                    OperationValue::Old => item,
                }
            }
        };

        // part 1 has us divide by 3, in part 2 we need to modulo by the LCM of the monkeys divisibility
        let item = match worry_divisor {
            None => item / 3,
            Some(x) => item % x,
        };

        //
        let result = item % self.divisible_by == 0;

        let throw_to = if result { self.if_true } else { self.if_false };

        (item, throw_to)
    }
}

fn round(monkeys: &mut Input, worry_divisor: Option<u64>) {
    for n in 0..monkeys.len() {
        // println!("Monkey {}:", n);
        let monkey = monkeys.get_mut(n).unwrap();

        let results = monkey.inspect_all(worry_divisor);

        // distribute to the other monkeys
        for (item, throw_to) in results {
            monkeys.get_mut(throw_to).unwrap().items.push(item);
        }
    }
}

fn print_monkeys(monkeys: &Input) {
    for m in monkeys {
        // let items: Vec<String> = m.items.iter().map(|x| x.to_string()).collect();
        // let items = items.join(", ");

        println!("Monkey {}: {}", m.number, m.inspected);
    }
}

fn get_monkey_business(monkeys: &mut Input) -> usize {
    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    monkeys[0].inspected * monkeys[1].inspected
}

fn problem1(monkeys: &mut Input) -> usize {
    for _n in 1..=20 {
        round(monkeys, None);
    }

    get_monkey_business(monkeys)
}

fn problem2(monkeys: &mut Input) -> usize {
    let lcm: u64 = monkeys.iter().map(|m| m.divisible_by).product();

    for _n in 1..=10000 {
        round(monkeys, Some(lcm));
        if [
            1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
        ]
        .contains(&_n)
        {
            println!("======= After round {_n} ======");
            print_monkeys(monkeys)
        }
    }

    get_monkey_business(monkeys)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let mut input = parse(input);
        let result = problem1(&mut input);
        assert_eq!(result, 10605)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let mut input = parse(input);
        let result = problem2(&mut input);
        assert_eq!(result, 2713310158)
    }
}
