use std::collections::{BTreeMap, VecDeque};

use advent_2023_20::broadcaster::Broadcaster;
use advent_2023_20::conjunction::Conjunction;
use advent_2023_20::flip_flop::FlipFlop;
use advent_2023_20::{ModuleKind, Pulse};

use common::math::lcm;
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

fn parse(input: &str) -> Input<'_> {
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

type ModuleMap<'a> = BTreeMap<&'a str, (ModuleKind, Vec<&'a str>)>;
/**
 * We can't create the actual Module instances during parsing because the
 * conjunction node needs to know all the incoming edges. So we have to have the entire
 * map in order to reverse the map and get the incoming edges so it can maintain state.
 */
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

fn problem2(input: &Input) -> i64 {
    /* You can see from the input that the graph is:
           (a,b,c,d) -> e -> rx

       So we only really need to pay attention to when e gets a High pulse from the incoming edges.
       I could have coded this up for my specific input, but where's the fun in that?
    */

    // find the module that goes into rx
    let incoming = input
        .iter()
        .find_map(|(name, (_id, targets))| targets.contains(&"rx").then_some(name))
        .cloned()
        .expect("Couldn't find an incoming module for rx");

    // find out how many modules send modules to that module
    let incoming_count = input
        .iter()
        .filter(|(_name, (_id, targets))| targets.contains(&incoming))
        .count();

    // actually create the modules
    let mut modules = create_modules(input);

    // keep track of the cycles of the incoming nodes
    let mut cycles_cache: BTreeMap<&str, u32> = BTreeMap::new();

    for presses in 1.. {
        let mut queue: VecDeque<(&str, Pulse, &str)> = VecDeque::new();
        queue.push_back(("button", Pulse::Low, "broadcaster"));

        while let Some((from, pulse, target)) = queue.pop_front() {
            // did we get a high pulse to the incoming node?
            if pulse == Pulse::High && target == incoming {
                cycles_cache.insert(from, presses);

                // do we have everything we need?
                if cycles_cache.len() == incoming_count {
                    // least common multiple the cycles
                    return cycles_cache
                        .values()
                        .map(|x| *x as i64)
                        .reduce(lcm)
                        .unwrap();
                }
            }

            if let Some((module, targets)) = modules.get_mut(target) {
                if let Some(result) = module.receive(from, pulse) {
                    for t in targets {
                        queue.push_back((target, result, t))
                    }
                }
            }
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
        let input = include_str!("../test1_interesting.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1)
    }
}
