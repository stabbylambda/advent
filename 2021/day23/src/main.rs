use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

use nom::{
    branch::alt,
    character::complete::{char, newline, one_of},
    combinator::map,
    multi::{many1, separated_list0},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Amphipods;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list0(
            newline,
            many1(alt((
                map(one_of(" .#"), |_| None),
                map(char('A'), |_| Some(Species::Amber)),
                map(char('B'), |_| Some(Species::Bronze)),
                map(char('C'), |_| Some(Species::Copper)),
                map(char('D'), |_| Some(Species::Desert)),
            ))),
        ),
        |input| {
            // all we really care about is the positions of the amphipods
            let amphipods: Vec<Species> = input
                .iter()
                .skip(2)
                .take(2)
                .flat_map(|row| row.iter().filter_map(|x| *x).collect_vec())
                .collect();

            Amphipods::new(&amphipods)
        },
    )(input);

    result.unwrap().1
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Species {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl Species {
    fn energy(&self) -> u32 {
        10_u32.pow(*self as u32)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]

struct Amphipods {
    hallway: [Option<Species>; 7], // there are only 7 hallway positions the amphipods can possibly go to
    rooms: [[Option<Species>; 2]; 4],
}

impl Amphipods {
    fn new(amphipods: &[Species]) -> Self {
        let mut s = Self::default();

        for x in 0..4 {
            s.rooms[x][1] = Some(amphipods[x]);
            s.rooms[x][0] = Some(amphipods[x + 4]);
        }

        s
    }
}

// https://topaz.github.io/paste/#XQAAAQB4DwAAAAAAAAA0m4iN4eJiGW+zE9q1pGPjM0CrVTb6vCSy7CrBJptvDNVXdmXAygHERk/k0YaUflHyCXRPOSSm6u85Q1zKAM+jf5xxj7jM94anOpe5L17fBoCOouWIZpQsPdMTGh2NF5Dhhlq64YSwoURpaBQOw1T0Efj5BQQqdmtV2U/THNOiHM9tbHKR8oc2UxDFwn5OA5YoD/MAxX6jaIqTSNqhxpUL9YqO9dXE3nJIrjKVx0mjOkGh2v/OPC4SoSc4GRDEjpn7kZNBhnmEKscgTt0w9TlCNDFiRfUyHMmlaKNei/GA75nXxEMHt9TVvs57WUpZkAyypnu4/cAa/RWmlOsomacGVhxj9TDSxRRIJ2Dc5n+QLQNshYVAiSENxU5EyxK/azYc48mTXEiPQHZkoi7IL0ErLZFvD1q3V75FAOV374I/qoEB4pZCPVUjFUYLSqXdQNTRsRbthuTK9uNudX8h4+D1enEUvSiFnfz7pVOZeb/C8ltNvHvqkBW02COnU49LOz5XPvu7OPUC9bxgB0uMqFLTt6VPyomvMEdUOwO6iG1RmtZQO25kl5+kvbL3ZoDlyyMVQTFa0LWQjbue0T3olGYByT+2XL7Kvv0y4/WU1TAPvfLDiRQPGMsIRBu/PehuJ7h1GkXPcU9EUhww4lcmqPBx/KFhQ6IbRO4wm2r0FRcQcBMarS6lO5crfF2woVH6moX20Jz8mIaIW1PtRS6QWHmoJJpupQxeQLpLdQfUoqRlw07JPRdruLon6KIloYnafwOmQt8A6f5QbZO/vyaz944RyhGNkuoHJEazA6/vyejRVaJpGzD58WfSlbI7xZe4RL7ZucMWAHuRkfJrRFE0qeBjrNmvPjC6/HzNWb9HKTapuLMg+d5+pUEWBA4eQcszKHtCyNqC+3MEVp4DITyz09iRSeS7toMQ8BQkEIHbl3AAPbdGpgF8sjwGkP/04VYSHdRPlotYtfHbNUhSVBPBY87WEHky2wBzv59FeXfOyFyT4M8frXpcrCaDoH4uIsUHMnDfprs/UbVlqAfXsbD9F+k0ZlJJJjfI/lPZ8n15AQAacpcHXy+cLWLKnhNG7Ie9BDDb7BtIk8426T8bngXWTR21WWxaLbZ1HO/sS0zegQ7rGmuyYJbS5XhfiABzpeGpf8d6C57DHLK7GcbdASspFBlza7uN1LH5Qrhnf5WFJEmIwX+Q5kTNHp9erVCMS9Vtde/fhCfwCkQpnkZbaC0GRXoYOrtT1/xPho93fYV+7FJjJu/fbXfkVxmavwppQEbXfpITHgjOynaOxeo3oiCmKSEV7b83AoSaHHrxlNoyiw+zAM82m6Lk4rpPTzrJnVUp0A3lzGo24BUUEVhXalvGi8az3BaSMF4CojWps7Psyz23zX3ysFUvFyS9U0Pz8FC9wx6hS0dRo6URGYaDlOfsVNTOWZ39PiJxzydfEpmynmOWADlleG6qfMg0dmAuE97O9PmmYOj3JhY8VgFBZA8qDAPvtR6F6vOOPS8ceTxbnouKKobCFI1AWbCf+rj3cioYszZWtVg8c7+2bw8QdwwMujBoYMbuWBoqXfs9w2rdCTZrUQmX/zTh1jjIFA19LDwR0fL/RdEqmUYRgYLYig4YN2NpyJFS1XH1RAVBDXlTKckc5rGcOG/LLcf8QbCWs19IrNjdWCpVOz1328LqJ6+JO9L/+9gbZA==
/* It's trivial to count the cost from the front space of the rooms to every valid hallway position:
 *
 * #############
 * #...........#
 * ###B#C#B#D###
 *   #A#D#C#A#
 *   #########
 */
const HALLWAY_COSTS: [[u32; 7]; 4] = [
    [3, 2, 2, 4, 6, 8, 9], // room A
    [5, 4, 2, 2, 4, 6, 7], // room B
    [7, 6, 4, 2, 2, 4, 5], // room C
    [9, 8, 6, 4, 2, 2, 3], // room D
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    amphipods: Amphipods,
    energy_spent: u32,
}

impl State {
    fn is_done(&self) -> bool {
        self.amphipods.rooms[0] == [Some(Species::Amber); 2]
            && self.amphipods.rooms[1] == [Some(Species::Bronze); 2]
            && self.amphipods.rooms[2] == [Some(Species::Copper); 2]
            && self.amphipods.rooms[3] == [Some(Species::Desert); 2]
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy_spent.cmp(&self.energy_spent)
    }
}

fn problem1(input: &Input) -> u32 {
    let mut queue = BinaryHeap::new();
    queue.push(State {
        amphipods: input.clone(),
        energy_spent: 0,
    });

    let mut cost_cache = HashMap::new();
    cost_cache.insert(input.clone(), 0);

    while let Some(state) = queue.pop() {
        if state.is_done() {
            return state.energy_spent;
        }
        dbg!(state.amphipods);
    }

    0
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
        assert_eq!(result, 0)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
