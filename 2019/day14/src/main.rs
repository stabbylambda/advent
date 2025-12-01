use std::collections::{HashMap, VecDeque};

use common::math::div_ceil;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u64},
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

type Input<'a> = HashMap<&'a str, (u64, Vec<(u64, &'a str)>)>;

fn parse(input: &str) -> Input<'_> {
    let ingredient = |s| separated_pair(u64, tag(" "), alpha1).parse(s);
    let rule = |s| {
        separated_pair(
            separated_list1(tag(", "), ingredient),
            tag(" => "),
            ingredient,
        ).parse(s)
    };
    let result: IResult<&str, Input> = map(separated_list1(newline, rule), |v| {
        v.into_iter()
            .map(|(components, (amount, production))| (production, (amount, components)))
            .collect()
    }).parse(input);

    result.unwrap().1
}

fn make_fuel(reactions: &Input, fuel: u64) -> u64 {
    let mut on_hand: HashMap<&str, u64> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut total_ore = 0;

    queue.push_back((fuel, "FUEL"));

    while let Some((needed, ingredient)) = queue.pop_front() {
        // we're here at the bottom, just total up the ore needed
        if ingredient == "ORE" {
            total_ore += needed;
            continue;
        }

        // consume as much of the leftovers as we can
        let leftover = on_hand.entry(ingredient).or_default();
        let remaining = if needed > *leftover {
            let remaining = needed - *leftover;
            *leftover = 0;
            Some(remaining)
        } else {
            *leftover -= needed;
            None
        };

        if let Some(remaining) = remaining {
            if let Some((production, ingredients)) = reactions.get(&ingredient) {
                // figure out the number of times we need to make this product
                let times = div_ceil(remaining as i64, *production as i64) as u64;

                // will we have new leftovers at the end of this production?
                // this working relies on us not having some weird need for this later down this tree
                if let Some(new_leftovers) = (production * times).checked_sub(remaining) {
                    let leftover = on_hand.entry(ingredient).or_default();
                    *leftover += new_leftovers;
                };

                // add all the sub products multiplied by the number of times we need to make them
                for (sub_amount, sub) in ingredients {
                    queue.push_back((sub_amount * times, sub))
                }
            }
        }
    }

    total_ore
}

fn problem1(reactions: &Input) -> u64 {
    make_fuel(reactions, 1)
}

fn problem2(reactions: &Input) -> u64 {
    let max_ore = 1_000_000_000_000u64;
    let ore_for_one = make_fuel(reactions, 1);

    let mut left = max_ore / ore_for_one; // instead of starting at zero, we obviously start at the lower bound of ore for one fuel
    let mut right = max_ore; // and obviously this is too far, but it's a big upper bound

    // classic binary search through the problem space to find the maximum fuel we can make with our ore
    while right.abs_diff(left) != 1 {
        let fuel = (right + left) / 2;
        if make_fuel(reactions, fuel) < max_ore {
            left = fuel;
        } else {
            right = fuel;
        }
    }

    left
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let cases = [
            (include_str!("../minimal.txt"), 30),
            (include_str!("../test1.txt"), 31),
            (include_str!("../test2.txt"), 165),
            (include_str!("../test3.txt"), 13312),
            (include_str!("../test4.txt"), 180697),
            (include_str!("../test5.txt"), 2210736),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem1(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn second() {
        let cases = [
            (include_str!("../test3.txt"), 82892753),
            (include_str!("../test4.txt"), 5586022),
            (include_str!("../test5.txt"), 460664),
        ];
        for (input, expected) in cases {
            let input = parse(input);
            let result = problem2(&input);
            assert_eq!(result, expected);
        }
    }
}
