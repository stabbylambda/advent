use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = common::read_input!();
    let input = parse(input);

    let (answer1, answer2) = problem(&input);
    println!("problem 1 answer: {answer1}");
    println!("problem 2 answer: {answer2}");
}

type Input = (Vec<bool>, HashMap<Vec<bool>, bool>);

fn parse(input: &str) -> Input {
    let plant = |s| alt((map(char('#'), |_| true), map(char('.'), |_| false))).parse(s);
    let plants = |s| many1(plant).parse(s);

    let result: IResult<&str, Input> = separated_pair(
        preceded(tag("initial state: "), plants),
        tag("\n\n"),
        map(
            separated_list1(newline, separated_pair(plants, tag(" => "), plant)),
            |x| x.iter().cloned().collect(),
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem((start, rules): &Input) -> (i64, i64) {
    const GENERATIONS: i64 = 50_000_000_000;
    let mut scores = vec![0];
    let mut diff_count: HashMap<i64, u64> = HashMap::new();

    // static padding of falses on either side so we don't run into weirdness
    let padding = 1000;
    let mut plants = vec![false; padding];
    plants.extend(start);
    plants.extend(vec![false; padding]);

    for generation in 1.. {
        // run the rules to produce the new generation
        let mut new_plants = plants.clone();
        for n in 2..plants.len() - 2 {
            let key = plants[n - 2..=n + 2].to_vec();
            let new_plant = *rules.get(&key).unwrap_or(&false);

            new_plants[n] = new_plant;
        }

        // reassign the plants
        plants = new_plants;

        // sum up all the pot indexes with plants in them
        let score = plants
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| (*x).then_some(idx))
            .map(|idx| idx as i64 - padding as i64)
            .sum();

        // get the last score
        let last_score = *scores.last().unwrap();
        let diff = score - last_score;

        // keep track of the number of times we've seen this diff
        let e = diff_count.entry(diff).and_modify(|x| *x += 1).or_insert(0);

        // once we've seen a diff enough times, we've got the final score, so bail
        if *e > 10 {
            let remaining_generations = GENERATIONS - generation;
            let final_score = score + (remaining_generations * diff);
            return (scores[20], final_score);
        }

        // put this score in the stack
        scores.push(score);
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let (result1, result2) = problem(&input);
        assert_eq!(result1, 325);
        assert_eq!(result2, 999_999_999_374);
    }
}
