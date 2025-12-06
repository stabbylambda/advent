use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use advent_2019_25::{parse_output, Door, Output, PressureSensorResult};
use itertools::Itertools;

use intcode::Intcode;

fn main() {
    let input = common::read_input!();
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");
}

type Input = Intcode;

#[derive(Clone)]
struct Droid {
    program: Intcode,
    inventory: HashSet<String>,
}

#[derive(Clone)]
enum DroidCommand {
    Move(Door),
    Take(String),
    Drop(String),
}

impl Display for DroidCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DroidCommand::Move(door) => writeln!(f, "{door}"),
            DroidCommand::Take(x) => writeln!(f, "take {x}"),
            DroidCommand::Drop(x) => writeln!(f, "drop {x}"),
        }
    }
}

impl Droid {
    fn new(intcode: &Intcode) -> Self {
        let intcode = intcode.clone();
        Self {
            program: intcode,
            inventory: HashSet::new(),
        }
    }

    fn execute_command(&mut self, command: Option<DroidCommand>) -> Option<Output> {
        let input = command.map(|x| x.to_string()).unwrap_or("".to_string());
        self.execute(&input)
    }

    fn execute(&mut self, input: &str) -> Option<Output> {
        let chars = Self::to_input_chars(input);
        self.program.input = chars;

        self.program.execute();
        let output: String = self
            .program
            .output
            .iter()
            .map(|x| (*x as u8) as char)
            .collect();
        self.program.output.clear();
        match parse_output(&output) {
            Ok((_, output)) => Some(output),
            _ => Some(Output::SomethingElse(output.to_string())),
        }
    }
    fn to_input_chars(s: &str) -> Vec<i64> {
        s.chars().map(|c| c as i64).rev().collect()
    }

    fn take_all_items(&mut self, items: &[String]) {
        // take every non-forbidden item in each room
        for item in items {
            if !FORBIDDEN.contains(&item.as_str()) {
                if let Some(Output::TakeItem(x)) =
                    self.execute_command(Some(DroidCommand::Take(item.to_string())))
                {
                    self.inventory.insert(x);
                }
            }
        }
    }
}

/* this is cheating but I didn't want to have to clone before taking the item and then keep track of
whether or not the droid halted
*/
const FORBIDDEN: [&str; 5] = [
    "infinite loop",
    "giant electromagnet",
    "photons",
    "molten lava",
    "escape pod",
];

fn auto_explore(droid: &mut Droid) {
    let mut seen: HashMap<(String, Door), u32> = HashMap::new();
    let mut input: Option<DroidCommand> = None;

    while let Some(output) = droid.execute_command(input.clone()) {
        match output {
            Output::PressureSensor(_, _) if droid.inventory.len() == 8 => {
                // once we've arrived at the pressure sensor with 8 items, we need to bail
                return;
            }
            // did we get to the room or the pressure sensor without 8 items?
            Output::InspectRoom(room) | Output::PressureSensor(_, Some(room)) => {
                // take everything in the current room
                droid.take_all_items(&room.items);

                // I originally kept track of moving through doors that you'd just been through,
                // but not doing that only leads to 22 extra moves...so...who cares?
                let next_door = room
                    .doors
                    .iter()
                    .map(|door| {
                        // get the times we've been through this door
                        let times = seen.get(&(room.name.to_string(), *door)).unwrap_or(&0);
                        (door, *times)
                    })
                    // go through the least travelled doors first, which helps us get out of infinite loops
                    .sorted_by_key(|(_, times)| *times)
                    .next();

                if let Some((door, _times)) = next_door {
                    // mark that we went through the door
                    seen.entry((room.name.to_string(), *door))
                        .and_modify(|times| *times += 1)
                        .or_insert(1);

                    input = Some(DroidCommand::Move(*door));
                } else {
                    unreachable!("We should never get in a spot where we can't travel");
                }
            }

            _ => unreachable!(),
        }
    }
}

fn problem1(input: &Input) -> u32 {
    let mut droid = Droid::new(input);
    // first explore the entire ship and take all the non-forbidden items you can find
    auto_explore(&mut droid);

    /* once we're here, we should be at the security checkpoint with 8 items
    go through the powerset of all our items, clone the droid, drop the ones in the list, and try to go through
    */
    droid
        .inventory
        .iter()
        .powerset()
        .find_map(|inventory_set| {
            let mut new_droid = droid.clone();
            // drop all these items
            for item in inventory_set {
                new_droid.execute_command(Some(DroidCommand::Drop(item.to_string())));
            }

            // go west through the door, if we got a password, we're done
            let result = new_droid.execute_command(Some(DroidCommand::Move(Door::West)));
            match result {
                Some(Output::PressureSensor(PressureSensorResult::Password(x), _)) => Some(x),
                _ => None,
            }
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::problem1;
    #[test]
    fn first() {
        let input = common::read_input!();
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 278664)
    }
}
