use std::collections::{BTreeMap, VecDeque};

use advent_2023_20::broadcaster::Broadcaster;
use advent_2023_20::conjunction::Conjunction;
use advent_2023_20::flip_flop::FlipFlop;
use advent_2023_20::{ModuleKind, Pulse};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input<'a> = BTreeMap<&'a str, (ModuleIdentifier, Vec<&'a str>)>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(
            newline,
            separated_pair(
                alt((
                    map(preceded(tag("%"), alpha1), |x| {
                        (x, ModuleIdentifier::FlipFlop)
                    }),
                    map(preceded(tag("&"), alpha1), |x| {
                        (x, ModuleIdentifier::Conjunction)
                    }),
                    map(tag("broadcaster"), |x| (x, ModuleIdentifier::Broadcaster)),
                )),
                tag(" -> "),
                separated_list1(tag(", "), alpha1),
            ),
        ),
        |x| {
            x.into_iter()
                .map(|((sender, kind), receivers)| (sender, (kind, receivers)))
                .collect()
        },
    )(input);

    result.unwrap().1
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleIdentifier {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

fn problem1(input: &Input) -> u32 {
    let mut modules = create_modules(input);

    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for _i in 0..1000 {
        let mut queue: VecDeque<(&str, Pulse, &str)> = VecDeque::new();
        queue.push_back(("button", Pulse::Low, "broadcaster"));

        while let Some((from, pulse, target)) = queue.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            };

            if let Some((module, targets)) = modules.get_mut(target) {
                if let Some(result) = module.receive(from, pulse) {
                    for t in targets {
                        queue.push_back((target, result, t))
                    }
                }
            }
        }
    }

    high_pulses * low_pulses
}

type ModuleMap<'a> = BTreeMap<&'a str, (ModuleKind, Vec<&'a str>)>;
fn create_modules<'a>(
    input: &BTreeMap<&'a str, (ModuleIdentifier, Vec<&'a str>)>,
) -> ModuleMap<'a> {
    let mut modules: ModuleMap = BTreeMap::new();

    // map all the incoming modules for conjunctions
    let conjunctions: Vec<(&&str, &Vec<&str>)> = input
        .iter()
        .filter_map(|(name, (k, targets))| {
            (*k == ModuleIdentifier::Conjunction).then_some((name, targets))
        })
        .collect();

    for (name, targets) in conjunctions {
        let incoming: Vec<&str> = input
            .iter()
            .filter_map(|(incoming_name, (_k, targets))| {
                targets.contains(name).then_some(*incoming_name)
            })
            .collect();

        modules.insert(
            *name,
            (
                ModuleKind::Conjunction(Conjunction::new(&incoming)),
                targets.clone(),
            ),
        );
    }

    // create the broadcaster
    modules.insert(
        "broadcaster",
        (
            ModuleKind::Broadcaster(Broadcaster::new()),
            input["broadcaster"].1.clone(),
        ),
    );

    // create the flipflops
    let flip_flops: Vec<_> = input
        .iter()
        .filter_map(|(name, (k, targets))| {
            (*k == ModuleIdentifier::FlipFlop).then_some((
                *name,
                (ModuleKind::FlipFlop(FlipFlop::new()), targets.clone()),
            ))
        })
        .collect();

    for (name, (module, targets)) in flip_flops {
        modules.insert(name, (module, targets));
    }

    modules
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 32000000)
    }
    #[test]
    fn first_interesting() {
        let input = include_str!("../test1_interesting.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 11687500)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
