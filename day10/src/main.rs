use std::collections::HashMap;

use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let mut input = parse(&input);

    let (score1, score2) = problem(&mut input, 17, 61);
    println!("problem 1 score: {score1}");
    println!("problem 2 score: {score2}");
}

type Input = HashMap<u32, Bot>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Value(u32);

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Output(u32),
    Bot(u32),
}

#[derive(Clone, Debug)]
struct Bot {
    number: u32,
    values: Vec<Value>,
    low: Instruction,
    high: Instruction,
}

impl Bot {
    fn give(&mut self, Value(v): &Value) {
        self.values.push(Value(*v));
    }

    fn compare(&mut self) -> Option<HandOut> {
        let [x, y] = &self.values[..] else {
            return None
        };

        let min = x.min(y);
        let max = x.max(y);

        let result = Some(HandOut {
            bot: self.number,
            min: *min,
            max: *max,
            low: self.low,
            high: self.high,
        });

        self.values.clear();

        result
    }
}

struct HandOut {
    bot: u32,
    min: Value,
    max: Value,
    low: Instruction,
    high: Instruction,
}

impl HandOut {
    fn contains(&self, value1: u32, value2: u32) -> bool {
        self.min == Value(value1) && self.max == Value(value2)
    }

    fn handouts(&self) -> Vec<(Value, Instruction)> {
        vec![(self.min, self.low), (self.max, self.high)]
    }
}

#[derive(Debug)]
enum Setup {
    Bot(Bot),
    Value(Value, u32),
}

fn parse(input: &str) -> Input {
    let starting_value = map(
        preceded(
            tag("value "),
            separated_pair(map(nom_u32, Value), tag(" goes to bot "), nom_u32),
        ),
        |(value, bot)| Setup::Value(value, bot),
    );

    let instruction = |input| {
        alt((
            map(preceded(tag("output "), nom_u32), Instruction::Output),
            map(preceded(tag("bot "), nom_u32), Instruction::Bot),
        ))(input)
    };

    let instructions = map(
        tuple((
            preceded(tag("bot "), nom_u32),
            preceded(tag(" gives low to "), instruction),
            preceded(tag(" and high to "), instruction),
        )),
        |(number, low, high)| {
            Setup::Bot(Bot {
                number,
                values: vec![],
                low,
                high,
            })
        },
    );

    let result: IResult<&str, Input> = map(
        separated_list1(newline, alt((starting_value, instructions))),
        |instructions| {
            let mut bots: HashMap<u32, Bot> = instructions
                .iter()
                .filter_map(|x| match x {
                    Setup::Bot(bot) => Some((bot.number, bot.clone())),
                    _ => None,
                })
                .collect();

            // hand out the values
            for x in instructions {
                let Setup::Value(v, bot) = x else {
                    continue;
                };

                bots.entry(bot).and_modify(|bot| bot.give(&v));
            }

            bots
        },
    )(input);

    result.unwrap().1
}

fn problem(bots: &mut Input, value1: u32, value2: u32) -> (u32, u32) {
    let mut bot_id = 0;
    let mut outputs: HashMap<u32, u32> = HashMap::new();

    loop {
        // find all the instructions we're about to hand out
        let handouts: Vec<HandOut> = bots
            .iter_mut()
            .filter_map(|(_num, bot)| bot.compare())
            .collect();

        //if no bots are handing anything out, then we're done
        if handouts.is_empty() {
            break;
        }

        // iterate over all the pairs
        for handout in handouts {
            // check for the one that we care about
            if handout.contains(value1, value2) {
                bot_id = handout.bot;
            }

            for (Value(v), m) in handout.handouts() {
                match m {
                    Instruction::Output(n) => {
                        outputs.entry(n).or_insert(v);
                    }
                    Instruction::Bot(bot) => {
                        bots.entry(bot).and_modify(|bot| bot.give(&Value(v)));
                    }
                }
            }
        }
    }

    (bot_id, outputs[&0] * outputs[&1] * outputs[&2])
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = get_raw_input();
        let mut input = parse(&input);
        let (bot_id, outputs) = problem(&mut input, 2, 5);
        assert_eq!(bot_id, 2);
        assert_eq!(outputs, 30);
    }
}
